use rand;
use std::fmt;
mod field;
mod network;

pub struct State {
	pub width: usize,
	pub cells: Vec<u8>,
}

impl State {
	// TODO: add adility to seed random engine.
	pub fn new(width: usize) -> Self {
		let mut cells = vec![0; width * width];

		for i in 0..cells.len() {
			let cell: bool = rand::random();
			cells[i] = cell as u8;
		}
		
		Self { width, cells }
	}

	pub fn tick(&mut self) {
		let f = &self.cells[..];
		let mut m: Vec<u8> = vec![0; f.len()];
		for i in 0..f.len() {
			let n = field::sum_adjacents(i, self.width, f);
			m[i] = match f[i] != 0 {
				true => (n > 1 && n < 4) as u8,
				false => (n == 3) as u8,
			};
		}
		self.cells = m;
	}

	pub fn to_vec(&self) -> Vec<u8> {
		network::pack(&self.cells[..])
	}
}

impl fmt::Debug for State {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for i in 0..self.width {
			for j in 0..self.width {
				write!(f, "{}", self.cells[i + j * self.width]).unwrap();
			}
			write!(f, "\n").unwrap();
		}
		Ok(())
	}
}
