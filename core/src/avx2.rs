use crate::vec256::I8x32;

#[cfg(target_arch = "x86")]
use core::arch::x86::*;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

const ALIVE_THREE_NEIGHBORS: I8x32 = I8x32::from_array([-125; 32]);
const ALIVE_TWO_NEIGHBORS: I8x32 = I8x32::from_array([-126; 32]);
const DEAD_THREE_NEIGHBORS: I8x32 = I8x32::from_array([3; 32]);

/// Apply Conway's rules to a vector of cell counters, returning packed cells.
///
/// Cell counter is a single byte structure, where leading bit (sign) defines
/// the initial state of the cell (`1` for `alive`. `0` otherwise) and the rest 
/// is used for counting `alive` heighbors (adjacent diagonally, horizontally 
/// and vertically). For example an `alive` cell with 8 `alive` neighbors looks
/// like: `1001000`.
///
/// The following rules are applied to each cell counter:
/// - An `alive` cell with 2 or 3 neighbors stays `alive`, `dead` otherwise;
/// - A `dead` cell with exactly 3 neighbors turns `alive`, `dead` otherwise;
///
/// Note: due to SIMD specifics, the bit order of the resulting bitmask is 
/// reversed. In order to preserve the order it can be reversed again after
/// packing, however, considering neihbors calculating logic, the original
/// vector of call counters is the one that should be reversed.
#[inline(always)]
unsafe fn apply_rules_and_pack(cells: __m256i) -> [u8; 4] {
    let alive_three = _mm256_cmpeq_epi8(cells, ALIVE_THREE_NEIGHBORS.into());
    let alive_two   = _mm256_cmpeq_epi8(cells, ALIVE_TWO_NEIGHBORS.into());
    let dead_three  = _mm256_cmpeq_epi8(cells, DEAD_THREE_NEIGHBORS.into());
    
    let res = _mm256_or_si256(alive_three, alive_two);
    let res = _mm256_or_si256(dead_three, res);

    // The mask is formed using leading bits of each byte
    core::mem::transmute(_mm256_movemask_epi8(res))
}

#[cfg(target_endian = "big")]
const BYTE_ORDER: I8x32 = I8x32::from_array([
    0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03,
    0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
]);

#[cfg(target_endian = "little")]
const BYTE_ORDER: I8x32 = I8x32::from_array([
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
    0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03,
]);

#[allow(overflowing_literals)]
const BIT_ORDER: I8x32 = I8x32::from_array([
    0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80,
    0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80,
    0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80,
    0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80,
]);


// /// Initialize cell counter 256 bit vector using packed cell data.
// ///
// /// Transform each bit of the provided packed cell data into a byte with sign
// /// bit corresponding to the cell state. This allows to count neighbors keeping
// /// initial state per each cell.
// #[inline]
// unsafe fn count_init(packed: i32) -> __m256i {
//     let bc = _mm256_set1_epi32(packed);
//     let ord = _mm256_shuffle_epi8(bc, BYTE_ORDER_MASK.into());
//     let inv = _mm256_and_si256(ord, BIT_POSITION_MASK.into());
//     let bi = _mm256_cmpeq_epi8(inv, BIT_POSITION_MASK.into());
//     _mm256_and_si256(bi, MSBIT_MASK.into())
// }

#[allow(overflowing_literals)]
const SIDES_BIT_POSITIONS: I8x32 = I8x32::from_array([
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
    0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
]);

#[allow(overflowing_literals)]
const SIDES_BYTE_ORDER: I8x32 = I8x32::from_array([
    0x0c, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
    0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
    0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
    0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x0f,
]);

const CELL_MASK: I8x32 = I8x32::from_array([0x01; 32]);

/// Generate a unit (row of packed 32 cells) of the next generation cells using
/// 9 units of the preceding generation. The resulting unit is located in the 
/// center of the provided chunk.
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn gen_avx2(
    t0: i32, t1: i32, t2: i32,
    m0: i32, m1: i32, m2: i32,
    b0: i32, b1: i32, b2: i32
) -> i32 {
    let sides = _mm256_set_epi32(t0, m0, b0, 0, t2, m2, b2, 0);
    println!("Sides original");
    for x in <__m256i as Into<I8x32>>::into(sides).iter() {
        print!("{:0>8b} ", x);
    }
    println!("");

    let sides = _mm256_and_si256(sides, SIDES_BIT_POSITIONS.into());
    let sides = _mm256_cmpeq_epi8(sides, SIDES_BIT_POSITIONS.into());
    let sides = _mm256_and_si256(sides, CELL_MASK.into());
    println!("Sides cleaned");
    for x in <__m256i as Into<I8x32>>::into(sides).iter() {
        print!("{:0>8b} ", x);
    }
    println!("");

    // Endianness is important for shift direction and consequent shuffle
    let sides_sl1 = _mm256_bslli_epi128(sides, 4);
    let sides_sl2 = _mm256_bslli_epi128(sides, 8);
    let sides = _mm256_add_epi8(sides, sides_sl1);
    let sides = _mm256_add_epi8(sides, sides_sl2);
    println!("Sides summed");
    for x in <__m256i as Into<I8x32>>::into(sides).iter() {
        print!("{:0>8b} ", x);
    }
    println!("");

    let sides = _mm256_shuffle_epi8(sides, SIDES_BYTE_ORDER.into());
    println!("Sides");
    for x in <__m256i as Into<I8x32>>::into(sides).iter() {
        print!("{:0>8b} ", x);
    }
    println!("");

    
    // let sides = _mm256_shuffle_epi8(sides, SIDES_BYTE_ORDER.into());
    // println!("Sides shuffled");
    // for x in <__m256i as Into<I8x32>>::into(sides).iter() {
    //     print!("{:0>8b} ", x);
    // }
    // println!("");
    //
    // let sides = _mm256_and_si256(sides, SIDES_BIT_POSITIONS.into());
    // println!("Sides Bits");
    // for x in <__m256i as Into<I8x32>>::into(sides).iter() {
    //     print!("{:0>8b} ", x);
    // }
    // println!("");
    //
    // let sides = _mm256_cmpeq_epi8(sides, SIDES_BIT_MASK.into());
    // println!("Sides bools");
    // for x in <__m256i as Into<I8x32>>::into(sides).iter() {
    //     print!("{:0>8b} ", x);
    // }
    // println!("");
    //
    // let sides = _mm256_and_si256(sides, LSBIT_MASK.into());
    // println!("Sides");
    // for x in <__m256i as Into<I8x32>>::into(sides).iter() {
    //     print!("{:0>8b} ", x);
    // }
    // println!("");
    //
    // let sides_l1 = _mm256_bslli_epi128(sides, 1);
    // println!("Sides shift left 1");
    // for x in <__m256i as Into<I8x32>>::into(sides_l1).iter() {
    //     print!("{:0>8b} ", x);
    // }
    // println!("");
    //
    // let sides_l2 = _mm256_bslli_epi128(sides, 2);
    // println!("Sides shift left 2");
    // for x in <__m256i as Into<I8x32>>::into(sides_l2).iter() {
    //     print!("{:0>8b} ", x);
    // }
    // println!("");
    //
    // let sides = _mm256_add_epi8(sides, sides_l1);
    // let sides = _mm256_add_epi8(sides, sides_l2);
    // println!("Sides sum");
    // for x in <__m256i as Into<I8x32>>::into(sides).iter() {
    //     print!("{:0>8b} ", x);
    // }
    // println!("");

    0
}

/// Generate a row of next generation cells using tree rows of the preceding
/// generation. The destination row located the same as the middle one.
///
/// Safety:
/// - Target should support `avx2` intristic;
/// - Lengths of `top`, `mid`, `bot` and `dst` should match;
/// - `top`, `mid` and `bot` should be initialized;
/// - `dst` should not overlap with any of `top`, `mid` or `bot`;
#[target_feature(enable = "avx2")]
#[inline]
pub unsafe fn gen_row_avx2(top: &[i32], mid: &[i32], bot: &[i32], dst: &mut [i32]) {
    let len = dst.len();

    dst.iter_mut().skip(1).take(len - 2)
        .zip(top.windows(3))
        .zip(mid.windows(3))
        .zip(bot.windows(3))
        .for_each(|(((dst, top), mid), bot)| {
            let (t0, t1, t2) = *(top.as_ptr() as *const (i32, i32, i32));
            let (m0, m1, m2) = *(mid.as_ptr() as *const (i32, i32, i32));
            let (b0, b1, b2) = *(bot.as_ptr() as *const (i32, i32, i32));

            *dst = gen_avx2(
                t0, t1, t2,
                m0, m1, m2,
                b0, b1, b2
            );
        });

    let (t0, t1) = *(top.as_ptr() as *const (i32, i32));
    let (t_p, t_n) = *(top.as_ptr().add(len - 2) as *const (i32, i32));

    let (m0, m1) = *(mid.as_ptr() as *const (i32, i32));
    let (m_p, m_n) = *(mid.as_ptr().add(len - 2) as *const (i32, i32));

    let (b0, b1) = *(bot.as_ptr() as *const (i32, i32));
    let (b_p, b_n) = *(bot.as_ptr().add(len - 2) as *const (i32, i32));

    // let dst0 = gen_avx2(
    //     t_n, t0, t1,
    //     m_n, m0, m1,
    //     b_n, b0, b1,
    // );
    // core::ptr::write(dst.as_mut_ptr() as *mut i32, dst0);

    // let dst_n = gen_avx2(
    //     t_p, t_n, t0,
    //     m_p, m_n, m0,
    //     b_p, b_n, b0,
    // );
    // core::ptr::write(dst.as_mut_ptr().add(len - 1) as *mut i32, dst_n);
}

#[cfg(test)]
mod test {
    use super::*;

    /// This tests components of `unpack` functons to confirm that the order
    /// of input bytes and the order of bits in this bytes (as each cell is 
    /// represented as a single bit when packed) is preserved.
    #[test]
    fn cell_order_preserved() {
        // When state is stored in memory and (probably) transferred over the 
        // wire, it contains cell data in comressed format, where each cell is 
        // represented by a single bit.
        let packed_cells: [u8; 4] = [0xf0, 0x2d, 0xaa, 0x18];

        let result: [u8; 4] = unsafe {
            let packed_cells = core::mem::transmute::<_, i32>(packed_cells);

            // Copy bytes (preserving order) 8 times filling the 256bit vector.
            let broadcasted = _mm256_set1_epi32(packed_cells);

            // Group 8 copies of the same byte together. The order of groups
            // is reversed relative to byte order of the input.
            let bytes_ordered = _mm256_shuffle_epi8(broadcasted, BYTE_ORDER.into());

            // Mask all bits of each bytes except for the one which index match 
            // byte's index in the group of 8.
            let bits_isolated = _mm256_and_si256(bytes_ordered, BIT_ORDER.into());

            // "Broadcasts" masked bit value to the rest bits in each byte.
            let bytes_as_bool = _mm256_cmpeq_epi8(bits_isolated, BIT_ORDER.into());

            // Build a `i32` value using sign bit of each byte. Bit order is 
            // reversed once again, restoring the original order.
            let mask = _mm256_movemask_epi8(bytes_as_bool);

            core::mem::transmute(mask)
        };

        assert_eq!(packed_cells, result);
    }

    #[test]
    fn rules_applied_correctly() {
        let cells = I8x32::from_array([
            -128, -127, -126, -125, -124, -123, -122, -121,
            -120, -119, -118, -117, -10, -9, -8, -7,
            -6, -5, -4, -3, -2, -1, 0, 1,
            2, 3, 4, 5, 6, 7, 8, 9,
        ]);
        let packed = unsafe { apply_rules_and_pack(cells.into()) };
        let expected = [0x0c, 0x00, 0x00, 0x02]; // Bit order is reversed

        assert_eq!(packed, expected);
    }

    #[test]
    fn gen_row_correct_result() {
        // let dst = &mut [0; 2];
        let dst = &mut [0; 3];

        let ch = crate::chunk![
            // "  ####  ####  #  # # # #  ##  #  # # ###     #     #  ####  ####"
            // " #     #  #  ## #  ####  ###   ##    #      #     #  ##    ##   "
            // "####  ####  #  #   # #  #  #  #    ###     ####  #  #     ####  "
            "######                                                                                         #"
            "#####                                                                                         ##"
            "####                                                                                         ###"
        ];

        let ex = crate::chunk![
            // "                                                                "
            // "     #      ### #  #   ###  # ### ##       #  #   ##### ##     #"
            // "                                                                "
            "######                                                                                         #"
            "#####                                                                                         ##"
            "####                                                                                         ###"
        ];

        unsafe {
            gen_row_avx2(&ch[0], &ch[1], &ch[2], dst);
        }

        assert_eq!(dst, &ex[1]);
    }
}
