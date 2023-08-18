use warp::Filter;
use tokio::sync::{broadcast};
use futures_util::{SinkExt, StreamExt};
use tokio::time::{sleep, Instant, Duration};
use convay;

#[tokio::main]
async fn main() {
	// Create a game tread, a thread safe client hashmap and a channel.
	// Convert the channel and the client hashmap into warp state and pass it
	// to the request handler, by moving the copy of mutex.
	let (tx, _rx) = broadcast::channel(16);
	let emitter = tx.clone();

	tokio::task::spawn(async move {
		let mut state = convay::State::new(512);

		loop {
			let timer = Instant::now();

			state.tick();
			println!("Tick {:#?}", timer.elapsed());

			tx.send(state.to_vec()).unwrap();
			println!("Send {:#?}", timer.elapsed());

			let dt = Duration::from_millis(90).saturating_sub(timer.elapsed());
			if !dt.is_zero() {
				sleep(dt).await;
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
            let result = tx.send(
            	warp::filters::ws::Message::binary(
            		sub.recv().await.unwrap()
            	)
            ).await;

            if result.is_err() {
            	// Release user resources and exit loop.
            	return;
            }
		}
    });	

	// Listen for incomming message, resovle the user using the hashmap,
	// decode the message (via network module) and turn into the action
	// object and send into game buffer (probably just a channel).
	while let Some(res) = rx.next().await {
		let msg = res.unwrap();
		println!("message received: {:?}", msg)
	}
}
