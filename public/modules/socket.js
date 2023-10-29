export default function () {
	const socket = new WebSocket(`ws://${window.location.host}`)
	socket.onerror = e => console.log('socket error', e)
	socket.onclose = e => console.log('socket closed', e)
	return socket
}

export async function decode({ data }) {
	// Instead of allocating a buffer each time data is received use
	// preallocated buffer instead. The buffer may be allocated on a handshake
	// (the server should send the size data) or on a specific message and then
	// reused.
	//
	// After the buffer size is known, the rendering buffer may be allocated
	// and fed into the renderer. Keep the process of bufferization and data
	// decoding decoupled (as later we may use WASM to compute game state).
	//
	// However the bufferization is necessary in order to release websocket,
	// as other messages (rather then game state update) may be introduced.
	// The socket is blocked until the state data is drained from it, so the
	// bufferization should be quick. (In case if buffer is locked by the
	// decoder, the tick may just be skipped and socket may be released).
	
	const bytes = new Uint8Array(await data.arrayBuffer())
	const buff = new Float32Array(8 * bytes.length)
	

	// This seems like a very inefficient way of doing this. TODO:: look into
	// video encoding, may be stream data just like the video instead (chunks?)
	let count = 0
	
	for (let i = 0; i < bytes.length; i++) {
		const b = bytes[i]
		for (let j = 0; j < 8; j++) {
			const alive = b >> j & 1
			if (alive) {
				buff[count] = i * 8 + (7 - j)
				count++
			}
		}
	
	}

	return [buff, count]
}
