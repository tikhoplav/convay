import Rendrer from './renderer.js'
import Socket, { decode } from './socket.js'

const canvas = document.getElementById('main')
const ctx = canvas.getContext('webgl2')
if (!ctx) throw Error('Canvas ctx unavailable')

const rend = Rendrer(ctx)


// TODO: This is just unreadable, please separate user actions, data pipeline
// and rendering logic, and leave only hookups in the main.js. Probably, some 
// Cotroller abstration better be introduced.
let scale = 10
let desiredScale = scale
rend.setTransform(0, 0, scale)
document.addEventListener('wheel', ({ deltaY }) => {
    const x = scale * (1 - Math.sign(deltaY) * 0.5)
    desiredScale = Math.min(Math.max(x, 1), 100)
})

window.onresize = () => {
    const { innerWidth: width, innerHeight: height} = window
    canvas.width = width
    canvas.height = height
    rend.resize()
}
window.onresize()

const socket = Socket()

const onFirst = async pack => {
    const [data, count] = await decode(pack)
    rend.createBuffer(data.length)
    rend.update(data, count) // so this `count`... It is here, because renderer
			     // needs to know exactly how much pixels should it 
			     // render... Disaster, no?
			     //
			     // There is an idea to render a bitmap texture on 


    socket.onmessage = async pack => {
	const [data, count] = await decode(pack)
	rend.update(data, count)
    }

    socket.removeEventListener('message', onFirst)
}
socket.addEventListener('message', onFirst)

const frame = dt => {
    if (Math.abs(scale - desiredScale) / scale > 0.01) {
	scale = scale - (scale - desiredScale) / 50
	rend.setTransform(0, 0, scale)
    }

    rend.draw()
    window.requestAnimationFrame(frame)
}

window.requestAnimationFrame(frame)

