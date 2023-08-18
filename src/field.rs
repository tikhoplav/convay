/// Converts cell's index into coordinates. `w` stands for the width of the
/// field's row. The field is assumed to be square form.
pub fn itoc(i: usize, w: usize) -> (u32, u32) {
	((i % w) as u32, (i / w) as u32)
}

/// Converts cell's coordinates into an index. `w` stands for the width of the
/// field's row. The field is assumed to be square form.
pub fn ctoi(x: u32, y: u32, w: usize) -> usize {
	(y as usize) * w + (x as usize)
}

/// Increase the coordinate (can be both x and y) by 1 and wrapping the result
/// around specified limit. Example: imagine having a square field 3 x 3 cells,
///   | | | |
///   | | |x|  <-- The cell of interest has the coordiantes x and y
///   | | | |
/// To find all neighbours of the cell which are to the right or bellow the
/// cell, it's coordinates needs to be calculated like (y + 1) and (x + 1). In
/// case showed above the x coordinate is the maximum possible, that the cell's
/// neighbours appears at the opposite side of the field (wrapped):
///   |a| | |
///   |a| |x|      `a` stands for adjacent
///   |a| | |
pub fn inc(c: u32, w: usize) -> u32 {
	(c + 1) % (w as u32)
}

/// Decrease the coordinate (can be both x and y) by 1 and wrapping the result
/// around specified limit. Example: imagine having a square field 3 x 3 cells,
///   | | | |
///   |x| | |  <-- The cell of interest has the coordiantes x and y
///   | | | |
/// To find all neighbours of the cell which are above or to the left from the
/// cell, it's coordinates needs to be calculated like (y - 1) and (x - 1). In
/// case showed above the x coordinate is 0, so left neighbours located at the
/// opposite side of the field (wrapped):
///   | | |a|
///   |x| |a|      `a` stands for adjacent
///   | | |a|
pub fn dec(c: u32, w: usize) -> u32 {
	let (cx, over) = c.overflowing_sub(1);
	(over as u32) * ((w - 1) as u32) + (!over as u32) * cx
}

/// Sum up values of cells adjacents to the selected one. In case if
/// cells only have two states (0 or 1) will return the number of non
/// zero cells neighbouring with the selected one.
pub fn sum_adjacents(i: usize, w: usize, f: &[u8]) -> u8 {
	let (x, y) = itoc(i, w);
	f[ctoi(dec(x, w), dec(y, w), w)] +
	f[ctoi(        x, dec(y, w), w)] +
	f[ctoi(inc(x, w), dec(y, w), w)] +
	f[ctoi(dec(x, w),         y, w)] +
	f[ctoi(inc(x, w),         y, w)] +
	f[ctoi(dec(x, w), inc(y, w), w)] +
	f[ctoi(        x, inc(y, w), w)] +
	f[ctoi(inc(x, w), inc(y, w), w)]
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn itoc_converts_index_to_coordinate() {
		let (x, y) = itoc(0, 8);
		assert_eq!(x, 0);
		assert_eq!(y, 0);

		let (x, y) = itoc(7, 8);
		assert_eq!(x, 7);
		assert_eq!(y, 0);

		let (x, y) = itoc(8, 8);
		assert_eq!(x, 0);
		assert_eq!(y, 1);

		let (x, y) = itoc(12, 8);
		assert_eq!(x, 4);
		assert_eq!(y, 1);

		let (x, y) = itoc(63, 8);
		assert_eq!(x, 7);
		assert_eq!(y, 7);
	}

	#[test]
	fn ctoi_coverts_coordinates_to_idx() {
		assert_eq!(ctoi(0, 0, 8), 0);
		assert_eq!(ctoi(3, 2, 8), 19);
		assert_eq!(ctoi(7, 7, 8), 63);
	}

	#[test]
	fn inc_wraps_around_limit() {
		assert_eq!(inc(7, 8), 0);
		assert_eq!(inc(4, 8), 5);
		assert_eq!(inc(0, 8), 1);
	}

	#[test]
	fn dec_wraps_around_limit() {
		assert_eq!(dec(7, 8), 6);
		assert_eq!(dec(1, 8), 0);
		assert_eq!(dec(0, 8), 7);
	}

	#[test]
	fn find_neightbours() {
		let w = 4;
		let f: &[u8] = &[
			0, 0, 0, 0,
			0, 1, 0, 1,
			0, 0, 0, 0,
			0, 1, 0, 1,
		];
		assert_eq!(sum_adjacents(0, w, f), 4);

		let f: &[u8] = &[
			0, 1, 0, 1,
			1, 0, 0, 0,
			0, 0, 0, 0,
			1, 0, 0, 0,
		];
		assert_eq!(sum_adjacents(0, w, f), 4);

		let f: &[u8] = &[
			0, 1, 0, 1,
			1, 1, 0, 1,
			0, 0, 0, 0,
			1, 1, 0, 1,
		];
		assert_eq!(sum_adjacents(0, w, f), 8);
	}
}
