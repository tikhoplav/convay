// #![no_std]

mod vec256;
mod fallback;

use core::{
    slice::from_raw_parts,
    iter::once,
};

/// Compute a row of next generation state using three rows of the precending
/// state. Location of the destination row correspond to the location of the 
/// middle row from the input.
///
/// # Panics
///
/// - Panics if any of the input slices and `dst` has unequal length;
#[inline]
pub fn gen_row(src: [&[i32]; 3], dst: &mut [i32]) {
    let len = dst.len();

    assert!(len > 1, "Length must be at least 2");
    assert!(len == src[0].len(), "Top row and dst length mismatch");
    assert!(len == src[1].len(), "Mid row and dst length mismatch");
    assert!(len == src[2].len(), "Bot row and dst length mismatch");

    // Safety:
    // Safe as all slices are proved to be valid for the whole `dst` length.
    unsafe {
        let dst0 = dst.as_mut_ptr() as *mut i32;
        let dst_last = dst.as_mut_ptr().add(len - 1) as *mut i32;

        let (top0, top1) = *(src[0].as_ptr() as *const (i32, i32));
        let (top_penultimate, top_last) = *(src[0].as_ptr().add(len - 2) as *const (i32, i32));
        let (mid0, mid1) = *(src[1].as_ptr() as *const (i32, i32));
        let (mid_penultimate, mid_last) = *(src[1].as_ptr().add(len - 2) as *const (i32, i32));
        let (bot0, bot1) = *(src[2].as_ptr() as *const (i32, i32));
        let (bot_penultimate, bot_last) = *(src[2].as_ptr().add(len - 2) as *const (i32, i32));

        let top_tup0 = from_raw_parts([top_last, top0, top1].as_ptr(), 3);
        let top_tup_last = from_raw_parts([top_penultimate, top_last, top0].as_ptr(), 3);
        let mid_tup0 = from_raw_parts([mid_last, mid0, mid1].as_ptr(), 3);
        let mid_tup_last = from_raw_parts([mid_penultimate, mid_last, mid0].as_ptr(), 3);
        let bot_tup0 = from_raw_parts([bot_last, bot0, bot1].as_ptr(), 3);
        let bot_tup_last = from_raw_parts([bot_penultimate, bot_last, bot0].as_ptr(), 3);

        dst.iter_mut().skip(1).take(len - 2)
            .zip(src[0].windows(3))
            .zip(src[1].windows(3))
            .zip(src[2].windows(3))
            .chain(
                once((((&mut *dst0, top_tup0), mid_tup0), bot_tup0))
            )
            .chain(
                once((((&mut *dst_last, top_tup_last), mid_tup_last), bot_tup_last))
            )
            .for_each(|(((dst, top), mid), bot)| {
                let (t0, t1, t2) = *(top.as_ptr() as *const (i32, i32, i32));
                let (m0, m1, m2) = *(mid.as_ptr() as *const (i32, i32, i32));
                let (b0, b1, b2) = *(bot.as_ptr() as *const (i32, i32, i32));

                *dst = fallback::gen(
                    t0, t1, t2,
                    m0, m1, m2,
                    b0, b1, b2
                );
            });
    }
}

// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// // #[cfg(target_feature = "avx2")]
// mod avx2;
//
// /// Generate a row of next generation cells using three rows of the preceding
// /// generation. The destination row location matches the location of the middle
// /// row from inputs.
// ///
// /// Uses optimized implementation if `avx2` feature is available on the target
// /// CPU. In order to preserve the interface for optimized and non-optimized 
// /// version the `i32` is used as a unit.
// ///
// /// # Panics
// ///
// /// - Panics if `top`, `mid`, `bot` and `dst` has different lengths;
// pub fn gen(top: &[i32], mid: &[i32], bot: &[i32], dst: &mut [i32]) {
//
//     #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
//     // #[cfg(target_feature = "avx2")]
//     return unsafe {
//         avx2::gen_row_avx2(top, mid, bot, dst)
//     };
//
//     // fallback_gen_row(top, mid, bot, dst)
// }
//
// #[inline]
// pub fn fallback_gen_row(top: &[i32], mid: &[i32], bot: &[i32], dst: &mut [i32]) {
//
// }

/// Convert an array of string literals into a two dimensional array of bytes 
/// (`[[u8; W]; H]`), mapping each character byte to a single bit, accroding to
/// the following rules:
/// - Printable ASCII characters, except for ` ` (space), `.` and `_`, are 
///   converted into `1` bit;
/// - The rest are converted into `0` bit;
///
/// # Note
///
/// The macros is processing individual bytes of the strings, that so the use 
/// of UTF8 characters may lead to unexpected results, as those may be encoded
/// with more than a single byte.
///
/// # Panics
///
/// - If strings have unequal lengths;
/// - If string length is not a modulo 8 (doesn't fit a whole number of bytes);
///
/// # Example
///
/// ```
/// let ch = conway::chunk![
///     "^aaa....bbbb....ccc$....`~~@________@~~`....$ddd....eeee....fff^"
///     " ^aaa....bbbb....ccc$....`~~@______@~~`....$ddd....eeee....fff^ "
///     "  ^aaa....bbbb....ccc$....`~~@____@~~`....$ddd....eeee....fff^  "
///     "   ^aaa....bbbb....ccc$....`~~@__@~~`....$ddd....eeee....fff^   "
///     "    ^aaa....bbbb....ccc$....`~~@@~~`....$ddd....eeee....fff^    "
///     "     ^aaa....bbbb....ccc$....`~~~~`....$ddd....eeee....fff^     "
///     "      ^aaa....bbbb....ccc$....`~~`....$ddd....eeee....fff^      "
/// ];
///
/// #[allow(overflowing_literals)]
/// let ints = [
///     [0xf0, 0xf0, 0xf0, 0xf0, 0x0f, 0x0f, 0x0f, 0x0f],
///     [0x78, 0x78, 0x78, 0x78, 0x1e, 0x1e, 0x1e, 0x1e],
///     [0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c],
///     [0x1e, 0x1e, 0x1e, 0x1e, 0x78, 0x78, 0x78, 0x78],
///     [0x0f, 0x0f, 0x0f, 0x0f, 0xf0, 0xf0, 0xf0, 0xf0],
///     [0x07, 0x87, 0x87, 0x87, 0xe1, 0xe1, 0xe1, 0xe0],
///     [0x03, 0xc3, 0xc3, 0xc3, 0xc3, 0xc3, 0xc3, 0xc0],
/// ];
///
/// assert_eq!(ch, ints);
/// ```
#[macro_export]
macro_rules! chunk {
    ( $($s:literal)* ) => {{
        const STRS: &[&'static [u8]] = &[ $($s.as_bytes(),)* ];
        assert!(STRS[0].len() % 8 == 0, "Width must be modulo 8");

        const W: usize = STRS[0].len() / 8;
        const H: usize = STRS.len();

        const CHUNK: [[u8; W]; H] = {
            let mut chunk = [[0; W]; H];
            let mut i = 0;
            while i < H {
                assert!(STRS[i].len() == 8 * W, "Unequal line length");
                let mut j = 0;
                while j < W {
                    let mut u = 0;
                    while u < 8 {
                        let bit = match STRS[i][8 * j + u] {
                            0x2e | 0x5f => 0,
                            0x21..=0x7e => 1,
                            _ => 0,
                        };
                        chunk[i][j] |= bit << (7 - u);
                        u += 1;
                    }
                    j += 1;
                }
                i += 1;
            }
            chunk
        };
        CHUNK
    }};
}

