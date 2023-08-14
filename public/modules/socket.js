export default function () {
	const socket = new WebSocket('ws://localhost:80')
	socket.onerror = e => console.log('socket error', e)
	socket.onclose = e => console.log('socket closed', e)
	return socket
}
