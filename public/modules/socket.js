export default function () {
	const socket = new WebSocket('ws://localhost:80')
	socket.onerror = e => console.log('socket error', e)
	socket.onclose = e => console.log('socket closed', e)
	return socket
}

export async function decode({ data }) {
	const bytes = new Uint8Array(await data.arrayBuffer())
	const map = new Float32Array(16 * bytes.length)
	for (let i = 0; i < bytes.length; i++) {
		const b = bytes[i]
		map[i * 16 +  0] = i * 8 + 0
		map[i * 16 +  1] = b >> 0 & 1
		map[i * 16 +  2] = i * 8 + 1
		map[i * 16 +  3] = b >> 1 & 1
		map[i * 16 +  4] = i * 8 + 2
		map[i * 16 +  5] = b >> 2 & 1
		map[i * 16 +  6] = i * 8 + 3
		map[i * 16 +  7] = b >> 3 & 1
		map[i * 16 +  8] = i * 8 + 4
		map[i * 16 +  9] = b >> 4 & 1
		map[i * 16 + 10] = i * 8 + 5
		map[i * 16 + 11] = b >> 5 & 1
		map[i * 16 + 12] = i * 8 + 6
		map[i * 16 + 13] = b >> 6 & 1
		map[i * 16 + 14] = i * 8 + 7
		map[i * 16 + 15] = b >> 7 & 1
	}
	return map
}
