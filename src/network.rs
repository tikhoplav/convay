// Game state treat every cell as a u8 to enable CPU cache and optimizations
// for state calculation, despite the fact that each cell contains only one bit
// of information. In terms of networking this requires 8 times more bandwidth
// to send the state over the wire. To minimize that, each 8 bytes of the game
// state are packed in one. This put the constraint on the game state that it's
// field's width has to be equal `n * 8`, where n is a natural number.
pub fn pack(f: &[u8]) -> Vec<u8> {
	let mut d: Vec<u8> = vec![0; f.len() / 8];
	for i in 0..d.len() {
		d[i] = f[i * 8 + 0] << 7 |
		       f[i * 8 + 1] << 6 |
		       f[i * 8 + 2] << 5 |
		       f[i * 8 + 3] << 4 |
		       f[i * 8 + 4] << 3 |
		       f[i * 8 + 5] << 2 |
		       f[i * 8 + 6] << 1 |
		       f[i * 8 + 7] << 0
	}
	d
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn pack_is_correct() {
		let f: &[u8] = &[
			0, 1, 0, 1, 1, 0, 0, 0,
			0, 0, 0, 0, 1, 0, 0, 0,
			1, 1, 1, 0, 0, 0, 1, 0,
			0, 0, 1, 0, 1, 0, 0, 1,
		];

		let bits = pack(f);
		assert_eq!(bits[0], 0b01011000u8);
		assert_eq!(bits[1], 0b00001000u8);
		assert_eq!(bits[2], 0b11100010u8);
		assert_eq!(bits[3], 0b00101001u8);
	}
}
