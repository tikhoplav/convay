export default function () {
	const socket = new WebSocket('ws://localhost:80')
	socket.onerror = e => console.log('socket error', e)
	socket.onclose = e => console.log('socket closed', e)
	return socket
}

export async function decode({ data }) {
	const bytes = new Uint8Array(await data.arrayBuffer())
	const buff = new Float32Array(8 * bytes.length)
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
