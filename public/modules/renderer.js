// TODO: instead of computing the scene based on individual cells, consider
// rendering the bit array onto a monochrome texture and apply it for a single
// polygon with scale and offset.
export default function Renderer(gl) {
    const vs = gl.createShader(gl.VERTEX_SHADER)
    const fs = gl.createShader(gl.FRAGMENT_SHADER)
    const prog = gl.createProgram()

    gl.shaderSource(vs, vertexShader)
    gl.shaderSource(fs, fragmentShader)

    gl.compileShader(vs)
    gl.compileShader(fs)

    gl.attachShader(prog, vs) 
    gl.attachShader(prog, fs)

    gl.linkProgram(prog)
    if (!gl.getProgramParameter(prog, gl.LINK_STATUS)) {
	console.error(`Link failed: ${gl.getProgramInfoLog(prog)}`);
	console.error(`vs info-log: ${gl.getShaderInfoLog(vs)}`);
	console.error(`fs info-log: ${gl.getShaderInfoLog(fs)}`);
	throw new Error("Failed to link a program");
    }

    gl.useProgram(prog)

    const uDimensions = gl.getUniformLocation(prog, 'uDimensions')
    const uResolution = gl.getUniformLocation(prog, 'uResolution')
    const uTransform = gl.getUniformLocation(prog, 'uTransform')

    gl.useProgram(null)

    let capacity
    let vbo
    let vao

    const createBuffer = size => {
	if (capacity >= size) return
	capacity = size

	gl.useProgram(prog)

	if (vbo) gl.deleteBuffer(vbo)

	vbo = gl.createBuffer()
	gl.bindBuffer(gl.ARRAY_BUFFER, vbo)
	gl.bufferData(gl.ARRAY_BUFFER, 8 * capacity, gl.DYNAMIC_DRAW)

	if (vao) gl.deleteVertexArray(vao)

	vao = gl.createVertexArray()
	gl.bindVertexArray(vao)

	gl.vertexAttribPointer(0, 1, gl.FLOAT, true, 4, 0)
	gl.enableVertexAttribArray(0)

	gl.bindVertexArray(null)

	const dim = Math.sqrt(capacity)
	gl.uniform2f(uDimensions, dim, dim)

	gl.bindBuffer(gl.ARRAY_BUFFER, null)
	gl.useProgram(null)
    }

    let points
    const update = (data, limit) => {
	gl.useProgram(prog)

	points = limit
	gl.bindBuffer(gl.ARRAY_BUFFER, vbo)

	gl.bufferSubData(gl.ARRAY_BUFFER, 0, data)
	gl.bindBuffer(gl.ARRAY_BUFFER, null)

	gl.useProgram(null)
    }

    const setTransform = (xOffset, yOffset, scale) => {
	gl.useProgram(prog)

	gl.uniform4f(uTransform, xOffset, yOffset, scale, 0)

	gl.useProgram(null)
    }

    const resize = () => {
	gl.useProgram(prog)

	const { width, height } = gl.canvas

	gl.viewport(0, 0, width, height)
	gl.uniform2f(uResolution, width, height)

	gl.useProgram(null)
    }

    const draw = () => {
	gl.useProgram(prog)
	gl.bindVertexArray(vao)

	gl.clearColor(0, 0, 0, 1)
	gl.clear(gl.COLOR_BUFFER_BIT)
	gl.enable(gl.DEPTH_TEST)

	gl.drawArrays(gl.POINTS, 0, points)

	gl.bindVertexArray(null)
	gl.useProgram(null)
    }

    return {
	createBuffer,
	setTransform,
	resize,
	update,
	draw,
    }
}

const vertexShader = `#version 300 es
uniform vec2 uDimensions;
uniform vec2 uResolution;
uniform vec4 uTransform;

layout(location = 0) in float idx;

void main() {
gl_PointSize = uTransform.z;

// Convert index into relative coordinates.
float x = mod(idx, uDimensions.x);
float y = -1.0 * floor(idx / uDimensions.x);
vec2 pos = 2.0 * vec2(x, y) / uDimensions + vec2(-1.0, 1.0);

// Offset cell to the center of it's square (pixel with width).
pos = pos + vec2(1.0, -1.0) / uDimensions;

// Scale and transform center of the field.
pos = (pos + vec2(uTransform)) * uTransform.z;

// Convert coordnates to screen space.
pos = pos * uDimensions / uResolution;

gl_Position = vec4(pos, 0, 1);
}`

const fragmentShader = `#version 300 es
precision highp float;
out vec4 diffuseColor;
void main() {
diffuseColor = vec4(1.0, 1.0, 1.0, 1.0);
}`
