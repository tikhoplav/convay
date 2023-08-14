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

	const vbo = gl.createBuffer()
	gl.bindBuffer(gl.ARRAY_BUFFER, vbo)
	gl.bufferData(gl.ARRAY_BUFFER, 512, gl.DYNAMIC_DRAW)

	const vao = gl.createVertexArray();
	gl.bindVertexArray(vao);

	gl.vertexAttribPointer(0, 1, gl.FLOAT, false, 8, 0);
	gl.enableVertexAttribArray(0);

	gl.vertexAttribPointer(1, 1, gl.FLOAT, false, 8, 4);
	gl.enableVertexAttribArray(1);

	gl.bindBuffer(gl.ARRAY_BUFFER, null)
	gl.bindVertexArray(null)
	gl.useProgram(null)

	const update = data => {
		gl.useProgram(prog)

		gl.bindBuffer(gl.ARRAY_BUFFER, vbo)
		gl.bufferSubData(gl.ARRAY_BUFFER, 0, data)
		gl.bindBuffer(gl.ARRAY_BUFFER, null)

		gl.useProgram(null)
	}

	const setDimensions = value => {
		gl.useProgram(prog)

		gl.uniform2f(uDimensions, value, value)

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

		gl.drawArrays(gl.POINTS, 0, 64)

		gl.bindVertexArray(null)
		gl.useProgram(null)
	}

	return {
		setDimensions,
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
layout(location = 1) in float alive;

out vec4 vColor;

void main() {
	// calculate cell position based on index
	float x = mod(idx, uDimensions.x);
	float y = floor(idx / uDimensions.x);

	// center the board at the origin, tanslating cells coordinates
	vec2 pos = vec2(x, y) - uDimensions / 2.0;

	// apply transform and scale
	pos = (pos + vec2(uTransform)) * uTransform.z;

	// each cell is a square based on current scale
	gl_PointSize = uTransform.z;

	// which means that position of each cell needs to be shifted
	// to place a cell into a center of it's square
	pos = pos + vec2(gl_PointSize, gl_PointSize) / 2.0;

	// convert position into a pixel space
	pos = pos * 2.0 / uResolution;

	gl_Position = vec4(pos, 0, 1);

	vColor = vec4(alive, alive, alive, 1);
}`

const fragmentShader = `#version 300 es
precision highp float;

in vec4 vColor;

out vec4 diffuseColor;

void main() {
  diffuseColor = vColor;
}`
