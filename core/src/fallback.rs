#[inline(always)]
pub fn gen(
    t0: i32, t1: i32, t2: i32,
    m0: i32, m1: i32, m2: i32,
    b0: i32, b1: i32, b2: i32
) -> i32 {
    let sides = [
        last(t0) + last(m0) + last(b0),
        0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
        first(t2) + first(m2) + first(b2)
    ];

    let counters = explode_counters(m1);
    let t1 = explode(t1);
    let m1 = explode(m1);
    let b1 = explode(b1);

    #[allow(overflowing_literals)]
    let counts = add([
        counters,
        sides,

        shift_left(t1),
        t1,
        shift_right(t1),

        shift_left(m1),
        shift_right(m1),

        shift_left(b1),
        b1,
        shift_right(b1),
    ]);

    apply_rules_and_pack(counts)
}

#[inline(always)]
const fn first(value: i32) -> i8 {
    let raw: *const i32 = &value;
    let raw = raw as *const i8;

    unsafe {
        #[cfg(target_endian = "big")]
        let raw = raw.offset(3);

        (*raw >> 7) & 0x01
    }
}

#[inline(always)]
fn last(value: i32) -> i8 {
    let raw: *const i32 = &value;
    let raw = raw as *const i8;

    unsafe {
        #[cfg(target_endian = "little")]
        let raw = raw.offset(3);

        *raw & 0x01
    }
}

#[inline(always)]
fn explode_counters(value: i32) -> [i8; 32] {
    let mut result = [0; 32];

    value.to_ne_bytes().iter().enumerate().for_each(|(i, src)| {
        for j in 0..8 {
            result[8 * i + j] = -128 * unsafe {
                core::mem::transmute::<_, i8>((src >> (7 - j)) & 1)
            };
        }
    });

    result
}

#[inline(always)]
fn explode(value: i32) -> [i8; 32] {
    let mut result = [0; 32];

    value.to_ne_bytes().iter().enumerate().for_each(|(i, src)| {
        for j in 0..8 {
            result[8 * i + j] = unsafe {
                core::mem::transmute::<_, i8>((src >> (7 - j)) & 1)
            };
        }
    });

    result
}

#[inline(always)]
fn shift_right(cells: [i8; 32]) -> [i8; 32] {
    let mut res = [0; 32];

    res.iter_mut().skip(1)
        .zip(cells)
        .for_each(|(dst, src)| {
            *dst = src;
        });

    res
}

#[inline(always)]
fn shift_left(cells: [i8; 32]) -> [i8; 32] {
    let mut res = [0; 32];
    
    res.iter_mut()
        .zip(cells.iter().skip(1))
        .for_each(|(dst, src)| {
            *dst = *src;
        });

    res
}

#[inline(always)]
const fn add<const N: usize, const W: usize>(values: [[i8; W]; N])-> [i8; W] {
    let (mut res, mut i) = ([0; W], 0);

    while i < W {
        let mut j = 0;
        while j < N {
            res[i] += values[j][i];
            j += 1;
        }
        i += 1;
    }

    res
}

#[inline(always)]
fn apply_rules_and_pack(cells: [i8; 32]) -> i32 {
    let mut res = [0u8; 4];

    cells.iter()
        .map(|byte| match byte {
            -125 | -126 | 3 => 1,
            _ => 0
        })
        .enumerate()
        .for_each(|(i, src)| {
            res[i / 8] |= src << (7 - i % 8);
        });

    unsafe { core::mem::transmute::<_, i32>(res) }
}

#[cfg(test)]
#[test]
fn test_gen() {
    let state = crate::chunk![
        r#".---..----..-..-..-.-.-..---..-..-.;.---. .---..---..-.-.-..---. .----..---. .-.   .-..---..---."#
        r#"| |  | || || .` || | | || | | >  /   \ \  | | _| | || | | || |-  | || || |-  | |__ | || |- | |- "#
        r#"`--- `---- `- `- `----- `-^-  `-    `---  `- -/`-^- `- - - `---  `---- `-    `---- `- `-   `--- "#
    ];
    let state = unsafe { core::mem::transmute::<_, [[i32; 3]; 3]>(state) };

    let next = crate::chunk![
        r#"................................................................................................"#
        r#"@....@........@.............@.@.@.@.....@.@....................@.@........@..@....@@.....@.@...."#
        r#"................................................................................................"#
    ];
    let next = unsafe { core::mem::transmute::<_, [[i32; 3]; 3]>(next) };

    let res = gen(
        state[0][2], state[0][0], state[0][1],
        state[1][2], state[1][0], state[1][1],
        state[2][2], state[2][0], state[2][1],
    );
    assert_eq!(res, next[1][0]);

    let res = gen(
        state[0][0], state[0][1], state[0][2],
        state[1][0], state[1][1], state[1][2],
        state[2][0], state[2][1], state[2][2],
    );
    assert_eq!(res, next[1][1]);

    let res = gen(
        state[0][1], state[0][2], state[0][0],
        state[1][1], state[1][2], state[1][0],
        state[2][1], state[2][2], state[2][0],
    );
    assert_eq!(res, next[1][2]);
}
