use warp::Filter;
use tokio::sync::broadcast;
use futures_util::{SinkExt, TryFutureExt, StreamExt};
use tokio::time::{sleep, Duration};
use rand::Rng;

#[tokio::main]
async fn main() {
	// Create a game tread, a thread safe client hashmap and a channel.
	// Convert the channel and the client hashmap into warp state and pass it
	// to the request handler, by moving the copy of mutex.
	let (tx, _rx) = broadcast::channel(16);
	let emitter = tx.clone();

	tokio::task::spawn(async move {
		loop {
			let now = std::time::SystemTime::now();

			sleep(Duration::from_millis(300)).await;

			{
				let mut rng = rand::thread_rng();
				let val: Vec<u8> = rng.gen::<f64>().to_le_bytes().to_vec();

				match now.elapsed() {
					Ok(elapsed) => {
						tx.send(val).unwrap();
						println!("Tick {:#?}", elapsed);
					},
					Err(e) => eprintln!("Failed to elapse: {}", e),
				};
			}
		}
	});

	let emitter = warp::any().map(move || emitter.clone());

	let socket = warp::path::end()
		.and(warp::ws())
		.and(emitter)
		.map(|ws: warp::ws::Ws, emitter: broadcast::Sender<Vec<u8>>| {
			let rx = emitter.subscribe();
			ws.on_upgrade(move |socket| on_connect(rx, socket))
		});

	let serve = warp::get().and(warp::fs::dir("/app/public"));
	let router = socket.or(serve);
	warp::serve(router).run(([0, 0, 0, 0], 80)).await;
}

async fn on_connect(mut sub: broadcast::Receiver<Vec<u8>>, socket: warp::ws::WebSocket) {
	let (mut tx, mut rx) = socket.split();

	// Listen for the incoming messages from the game channel, when received
	// convert pack it into a network message (probably via a network module)
	// and send it to a client. If message is private find the client using
	// the hashmap.
	tokio::task::spawn(async move {
		loop {
			let msg = sub.recv().await.unwrap();
			let b = warp::filters::ws::Message::binary(msg);
            tx.send(b).unwrap_or_else(|e| {
				eprintln!("error writing to socket: {}", e);
			}).await;
		}
    });	

	// Listen for incomming message, resovle the user using the hashmap,
	// decode the message (via network module) and turn into the action
	// object and send into game buffer (probably just a channel).
	while let Some(res) = rx.next().await {
		let msg = match res {
			Ok(msg) => msg,
			Err(e) => {
				eprintln!("error reading from socket: {}", e);
				break;
			}
		};
		println!("message received: {:?}", msg)
	}

	// Handle client disconnection, release resources.
}
