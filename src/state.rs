//! Bit state

use core::iter::once;
use crate::rand::fill;

/// Conway's game of Life state.
///
/// Represents a 2D grid with cells of two variants: dead or alive.
/// Each set of 8 horizontally consecutive cells is packed into a single
/// byte (a `strip`);
pub type State<const W: usize, const H: usize> = [[u8; W]; H];

/// Create an empty game state of sertain dimensions.
///
/// A state created with this function is guarantied to be usable with
/// `tick` function as it performs a following checks:
/// - Height of the state must be modulo 8;
/// - Width of the state must be modulo 8 (64 cells);
pub const fn new_state<const W: usize, const H: usize>() -> State<W, H> {
    assert!(H % 8 == 0, "Height must be modulo 8");
    assert!(W % 64 == 0, "Width must be modulo 8");
    [[0u8; W]; H]
}

/// Create a randomly filled state
#[cfg(feature = "rand")]
pub fn random_state<const W: usize, const H: usize>(seed: &[u8]) -> State<W, H> {
    let mut state: State<W, H> = new_state();
    fill(&mut state, seed);
    state
}

/// Generate a new game state applying Rules to the previous state.
pub fn tick<const W: usize, const H: usize>(prev: &State<W, H>, next: &mut State<W, H>) {
    for h in (0..H).step_by(8) {
        for w in (0..W).step_by(8) {
            // In order to miminize the number of write ops into the next
            // state, which most likely is heap allocated, the chunk of
            // 8x8 strips is allocated at the stack. Considering each strip
            // length of one byte, the resulting 64 bytes fits exactly one
            // CPU storage slot.
            let mut chunk = [[0u8; 8]; 8];

            // A chunk of strips of any given size NxM has enough data to
            // compute a chunk of strips of a new state of the same size
            // except for the borders. For that reason bands of 10x4 strips
            // are used to generate a single chunks in four steps:
            //
            // . . . . . . . . . .
            // . x x x x x x x x .      A quarter of the new state chunk is
            // . x x x x x x x x .      computed using a single 10x4 band
            // . . . . . . . . . .
            //
            // Since bands are using overlapping regions from the previous
            // state, it worth storing bands in two halves, that so the second
            // half can be reused for the next computation.

            let y0 = h.checked_sub(1).unwrap_or(H - 1);
            let yn = (h + 8) % H;

            let x0 = w.checked_sub(1).unwrap_or(W - 1);
            let xn = (w + 8) % W;

            let mut band_a = [[0u8; 10]; 2];
            [y0, h].iter().zip(band_a.iter_mut()).for_each(|(y, row)| {
                once(x0)
                    .chain(w..w + 8)
                    .chain(once(xn))
                    .zip(row.iter_mut())
                    .for_each(|(x, dst)| {
                        *dst = prev[*y][x];
                    })
            });

            let mut band_b = [[0u8; 10]; 2];
            [h + 1, h + 2]
                .iter()
                .zip(band_b.iter_mut())
                .for_each(|(y, row)| {
                    once(x0)
                        .chain(w..w + 8)
                        .chain(once(xn))
                        .zip(row.iter_mut())
                        .for_each(|(x, dst)| {
                            *dst = prev[*y][x];
                        })
                });

            seg(&mut chunk[..2], &band_a, &band_b);

            [h + 3, h + 4]
                .iter()
                .zip(band_a.iter_mut())
                .for_each(|(y, row)| {
                    once(x0)
                        .chain(w..w + 8)
                        .chain(once(xn))
                        .zip(row.iter_mut())
                        .for_each(|(x, dst)| {
                            *dst = prev[*y][x];
                        })
                });

            seg(&mut chunk[2..4], &band_b, &band_a);

            [h + 5, h + 6]
                .iter()
                .zip(band_b.iter_mut())
                .for_each(|(y, row)| {
                    once(x0)
                        .chain(w..w + 8)
                        .chain(once(xn))
                        .zip(row.iter_mut())
                        .for_each(|(x, dst)| {
                            *dst = prev[*y][x];
                        })
                });

            seg(&mut chunk[4..6], &band_a, &band_b);

            [h + 7, yn]
                .iter()
                .zip(band_a.iter_mut())
                .for_each(|(y, row)| {
                    once(x0)
                        .chain(w..w + 8)
                        .chain(once(xn))
                        .zip(row.iter_mut())
                        .for_each(|(x, dst)| {
                            *dst = prev[*y][x];
                        })
                });

            seg(&mut chunk[6..], &band_b, &band_a);

            next[h..h + 8]
                .iter_mut()
                .zip(chunk.iter())
                .for_each(|(next, row)| {
                    next[w..w + 8]
                        .iter_mut()
                        .zip(row.iter())
                        .for_each(|(next, strip)| {
                            *next = *strip;
                        })
                });
        }
    }
}

/// Compute a 8x2 segment of strips of the new generation using bands
///
/// The band is a 10x2 area, the segment is computed using two band,
/// located one below another:
///
/// a a a a a a a a a a        . . . . . . . . . .
/// a a a a a a a a a a   =>   . x x x x x x x x .
/// b b b b b b b b b b        . x x x x x x x x .
/// b b b b b b b b b b        . . . . . . . . . .
///
/// The band size were selected in order to fit two band (40 bytes) into
/// a CPU cache slot. However, in order to optimize more, the lower band
/// can be used to compute next segment of the chunk.
#[inline]
fn seg(chunk: &mut [[u8; 8]], a: &[[u8; 10]; 2], b: &[[u8; 10]; 2]) {
    chunk[0]
        .iter_mut()
        .zip(a[0].windows(3))
        .zip(a[1].windows(3))
        .zip(b[0].windows(3))
        .for_each(|(((dst, top), mid), bot)| {
            *dst = strip(
                top[0], top[1], top[2], mid[0], mid[1], mid[2], bot[0], bot[1], bot[2],
            );
        });

    chunk[1]
        .iter_mut()
        .zip(a[1].windows(3))
        .zip(b[0].windows(3))
        .zip(b[1].windows(3))
        .for_each(|(((dst, top), mid), bot)| {
            *dst = strip(
                top[0], top[1], top[2], mid[0], mid[1], mid[2], bot[0], bot[1], bot[2],
            );
        });
}

/// Compute a value of the next generation "strip" (a row of 8 cells)
///
/// Provided values are strips from previous generation, located accoring
/// to the scheme below:
///
/// tl  t  tr
///  l  m  r      where target strip located in the state at `m` position
/// bl  b  br
#[inline(always)]
const fn strip(tl: u8, t: u8, tr: u8, l: u8, m: u8, r: u8, bl: u8, b: u8, br: u8) -> u8 {
    (((((m >> 7) & 1)
        * (((tl & 1)
            + ((t >> 7) & 1)
            + ((t >> 6) & 1)
            + (l & 1)
            + ((m >> 6) & 1)
            + (bl & 1)
            + ((b >> 7) & 1)
            + ((b >> 6) & 1))
            == 2
            || ((tl & 1)
                + ((t >> 7) & 1)
                + ((t >> 6) & 1)
                + (l & 1)
                + ((m >> 6) & 1)
                + (bl & 1)
                + ((b >> 7) & 1)
                + ((b >> 6) & 1))
                == 3) as u8)
        + ((1 - ((m >> 7) & 1))
            * (((tl & 1)
                + ((t >> 7) & 1)
                + ((t >> 6) & 1)
                + (l & 1)
                + ((m >> 6) & 1)
                + (bl & 1)
                + ((b >> 7) & 1)
                + ((b >> 6) & 1))
                == 3) as u8))
        << 7)
        + (((((m >> 6) & 1)
            * ((((t >> 7) & 1)
                + ((t >> 6) & 1)
                + ((t >> 5) & 1)
                + ((m >> 7) & 1)
                + ((m >> 5) & 1)
                + ((b >> 7) & 1)
                + ((b >> 6) & 1)
                + ((b >> 5) & 1))
                == 2
                || (((t >> 7) & 1)
                    + ((t >> 6) & 1)
                    + ((t >> 5) & 1)
                    + ((m >> 7) & 1)
                    + ((m >> 5) & 1)
                    + ((b >> 7) & 1)
                    + ((b >> 6) & 1)
                    + ((b >> 5) & 1))
                    == 3) as u8)
            + ((1 - ((m >> 6) & 1))
                * ((((t >> 7) & 1)
                    + ((t >> 6) & 1)
                    + ((t >> 5) & 1)
                    + ((m >> 7) & 1)
                    + ((m >> 5) & 1)
                    + ((b >> 7) & 1)
                    + ((b >> 6) & 1)
                    + ((b >> 5) & 1))
                    == 3) as u8))
            << 6)
        + (((((m >> 5) & 1)
            * ((((t >> 6) & 1)
                + ((t >> 5) & 1)
                + ((t >> 4) & 1)
                + ((m >> 6) & 1)
                + ((m >> 4) & 1)
                + ((b >> 6) & 1)
                + ((b >> 5) & 1)
                + ((b >> 4) & 1))
                == 2
                || (((t >> 6) & 1)
                    + ((t >> 5) & 1)
                    + ((t >> 4) & 1)
                    + ((m >> 6) & 1)
                    + ((m >> 4) & 1)
                    + ((b >> 6) & 1)
                    + ((b >> 5) & 1)
                    + ((b >> 4) & 1))
                    == 3) as u8)
            + ((1 - ((m >> 5) & 1))
                * ((((t >> 6) & 1)
                    + ((t >> 5) & 1)
                    + ((t >> 4) & 1)
                    + ((m >> 6) & 1)
                    + ((m >> 4) & 1)
                    + ((b >> 6) & 1)
                    + ((b >> 5) & 1)
                    + ((b >> 4) & 1))
                    == 3) as u8))
            << 5)
        + (((((m >> 4) & 1)
            * ((((t >> 5) & 1)
                + ((t >> 4) & 1)
                + ((t >> 3) & 1)
                + ((m >> 5) & 1)
                + ((m >> 3) & 1)
                + ((b >> 5) & 1)
                + ((b >> 4) & 1)
                + ((b >> 3) & 1))
                == 2
                || (((t >> 5) & 1)
                    + ((t >> 4) & 1)
                    + ((t >> 3) & 1)
                    + ((m >> 5) & 1)
                    + ((m >> 3) & 1)
                    + ((b >> 5) & 1)
                    + ((b >> 4) & 1)
                    + ((b >> 3) & 1))
                    == 3) as u8)
            + ((1 - ((m >> 4) & 1))
                * ((((t >> 5) & 1)
                    + ((t >> 4) & 1)
                    + ((t >> 3) & 1)
                    + ((m >> 5) & 1)
                    + ((m >> 3) & 1)
                    + ((b >> 5) & 1)
                    + ((b >> 4) & 1)
                    + ((b >> 3) & 1))
                    == 3) as u8))
            << 4)
        + (((((m >> 3) & 1)
            * ((((t >> 4) & 1)
                + ((t >> 3) & 1)
                + ((t >> 2) & 1)
                + ((m >> 4) & 1)
                + ((m >> 2) & 1)
                + ((b >> 4) & 1)
                + ((b >> 3) & 1)
                + ((b >> 2) & 1))
                == 2
                || (((t >> 4) & 1)
                    + ((t >> 3) & 1)
                    + ((t >> 2) & 1)
                    + ((m >> 4) & 1)
                    + ((m >> 2) & 1)
                    + ((b >> 4) & 1)
                    + ((b >> 3) & 1)
                    + ((b >> 2) & 1))
                    == 3) as u8)
            + ((1 - ((m >> 3) & 1))
                * ((((t >> 4) & 1)
                    + ((t >> 3) & 1)
                    + ((t >> 2) & 1)
                    + ((m >> 4) & 1)
                    + ((m >> 2) & 1)
                    + ((b >> 4) & 1)
                    + ((b >> 3) & 1)
                    + ((b >> 2) & 1))
                    == 3) as u8))
            << 3)
        + (((((m >> 2) & 1)
            * ((((t >> 3) & 1)
                + ((t >> 2) & 1)
                + ((t >> 1) & 1)
                + ((m >> 3) & 1)
                + ((m >> 1) & 1)
                + ((b >> 3) & 1)
                + ((b >> 2) & 1)
                + ((b >> 1) & 1))
                == 2
                || (((t >> 3) & 1)
                    + ((t >> 2) & 1)
                    + ((t >> 1) & 1)
                    + ((m >> 3) & 1)
                    + ((m >> 1) & 1)
                    + ((b >> 3) & 1)
                    + ((b >> 2) & 1)
                    + ((b >> 1) & 1))
                    == 3) as u8)
            + ((1 - ((m >> 2) & 1))
                * ((((t >> 3) & 1)
                    + ((t >> 2) & 1)
                    + ((t >> 1) & 1)
                    + ((m >> 3) & 1)
                    + ((m >> 1) & 1)
                    + ((b >> 3) & 1)
                    + ((b >> 2) & 1)
                    + ((b >> 1) & 1))
                    == 3) as u8))
            << 2)
        + (((((m >> 1) & 1)
            * ((((t >> 2) & 1)
                + ((t >> 1) & 1)
                + (t & 1)
                + ((m >> 2) & 1)
                + (m & 1)
                + ((b >> 2) & 1)
                + ((b >> 1) & 1)
                + (b & 1))
                == 2
                || (((t >> 2) & 1)
                    + ((t >> 1) & 1)
                    + (t & 1)
                    + ((m >> 2) & 1)
                    + (m & 1)
                    + ((b >> 2) & 1)
                    + ((b >> 1) & 1)
                    + (b & 1))
                    == 3) as u8)
            + ((1 - ((m >> 1) & 1))
                * ((((t >> 2) & 1)
                    + ((t >> 1) & 1)
                    + (t & 1)
                    + ((m >> 2) & 1)
                    + (m & 1)
                    + ((b >> 2) & 1)
                    + ((b >> 1) & 1)
                    + (b & 1))
                    == 3) as u8))
            << 1)
        + (((m & 1)
            * ((((t >> 1) & 1)
                + (t & 1)
                + ((tr >> 7) & 1)
                + ((m >> 1) & 1)
                + ((r >> 7) & 1)
                + ((b >> 1) & 1)
                + (b & 1)
                + ((br >> 7) & 1))
                == 2
                || (((t >> 1) & 1)
                    + (t & 1)
                    + ((tr >> 7) & 1)
                    + ((m >> 1) & 1)
                    + ((r >> 7) & 1)
                    + ((b >> 1) & 1)
                    + (b & 1)
                    + ((br >> 7) & 1))
                    == 3) as u8)
            + ((1 - (m & 1))
                * ((((t >> 1) & 1)
                    + (t & 1)
                    + ((tr >> 7) & 1)
                    + ((m >> 1) & 1)
                    + ((r >> 7) & 1)
                    + ((b >> 1) & 1)
                    + (b & 1)
                    + ((br >> 7) & 1))
                    == 3) as u8))
}

/// Generate a 8x2 segment of the new state using two 10x2 bands
// const fn seg(rows: &mut [[u8; 8]], a: &[[u8; 10]; 2], b: &[[u8; 10]; 2]) {
//
// }

/// Create a tick`able chunk from string literals
///
/// When string is characters a devided in two groups by their ascii codes:
/// - Even - treated as alive cells;
/// - Odd  - treated as dead cells;
///
/// Macro throws compile time errors if valid state can not be created from
/// the provided input, otherwise the resulting chunk / state is guaranteed to
/// be valid.
///
/// ## Example
///
/// ```
/// use conway::chunk;
///
/// let chunk = chunk!(
///     "0000000000000000000000000000000000000000000000000000000000000000"
///     "0000000000000000000000000000000000000000000000000000000000000000"
///     "0000000000000000000000000000000000000000000000000000000000000000"
///     "0000000000000000000000000000000000000000000000000000000000000000"
///     "00000000000     000000000000000000000000000000000000000000000000"
///     "00000000000 ..O 000000000000000000000000000000000000000000000000"
///     "00000000000 O.1 000000000000000000000000000000000000000000000000"
///     "00000000000  11 000000000000000000000000000000000000000000000000"
///     "00000000000     000000000000000000000000000000000000000000000000"
///     "0000000000000000000000000000000000000000000000000000000000000000"
///     "0000000000000000000000000000000000000000000000000000000000000000"
///     "0000000000000000000000000000000000000000000000000000000000000000"
///     "0000000000000000000000000000000000000000000000000000000000000000"
///     "0000000000000000000000000000000000000000000000000000000000000000"
///     "0000000000000000000000000000000000000000000000000000000000000000"
///     "0000000000000000000000000000000000000000000000000000000000000000"
/// );
///
/// assert_eq!(chunk[5][1], 0b00000010);
/// assert_eq!(chunk[6][1], 0b00001010);
/// assert_eq!(chunk[7][1], 0b00000110);
/// ```
#[macro_export]
macro_rules! chunk {
    ( $($s:literal)* ) => {{
        const STRS: &[&'static [u8]] = &[$($s.as_bytes(),)*];

        assert!(STRS.len() % 8 == 0, "Height must be modulo 8");
        assert!(STRS[0].len() % 64 == 0, "Width must be modulo 64");

        const W: usize = STRS[0].len() / 8;
        const H: usize = STRS.len();

        const CHUNK: [[u8; W]; H] = {
            let mut chunk = [[0u8; W]; H];

            let mut h = 0;
            while h < H {
                assert!(STRS[h].len() == W * 8, "Unequal row length");

                let mut w = 0;
                while w < W {
                    let mut u = 0;
                    while u < 8 {
                        chunk[h][w] += (((STRS[h][w * 8 + u] & 1) as u16) << (7 - u)) as u8;
                        u += 1;
                    }
                    w += 1;
                }
                h += 1;
            }

            chunk
        };

        CHUNK
    }};
}

#[cfg(test)]
mod test {
    use super::{seg, strip, tick, State};

    #[test]
    fn comp_strip() {
        assert_eq!(129, strip(0, 128, 128, 0, 128, 128, 1, 0, 128));
    }

    #[test]
    fn comp_segment() {
        let a: [[u8; 10]; 2] = [
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        let b: [[u8; 10]; 2] = [
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        let mut chunk = [[0u8; 8]; 8];
        seg(&mut chunk[..2], &a, &b);

        let expected: [[u8; 8]; 2] = [[128, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0]];
        assert_eq!(expected, chunk[..2]);
    }

    #[test]
    fn glider_test() {
        let mut prev: State<8, 32> = chunk!(
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "......................O........................................."
            "....................O.O........................................."
            ".....................OO........................................."
            "................................................................"
            "................................................................"
            "................................................................"
            ".................................OO............................."
            ".................................OO............................."
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
        );

        let mut next: State<8, 32> = [[0u8; 8]; 32];

        for _ in 0..8 {
            tick(&prev, &mut next);
            tick(&next, &mut prev);
        }

        let expected: State<8, 32> = chunk!(
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "..........................O....................................."
            "........................O.O....................................."
            ".........................OO......OO............................."
            ".................................OO............................."
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
            "................................................................"
        );

        assert_eq!(prev, expected);
    }
}
