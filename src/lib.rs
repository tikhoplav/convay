#[derive(PartialEq)]
#[repr(transparent)]
pub struct Cell(i8);

impl core::fmt::Debug for Cell {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl core::fmt::Display for Cell {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 < 0 {
            true => write!(f, "#"),
            _ => write!(f, " "),
        }
    }
}

impl Cell {
    /// Evaluates the cell's state based on accumulated surroundings data.
    ///
    /// This applies Conway's rules to the cell, evaluating it's next state
    /// based on the current and surroundings data. It's important to record
    /// the surroundings data (all 8 adjacent cells) prior calling this.
    ///
    /// Produces a `u8` equivalent of `bool`, to mitigate the need of further
    /// type casting.
    #[inline(always)]
    pub const fn eval(&self) -> u8 {
        match self.0 {
            -125 | -126 | 3 => 1,
            _ => 0,
        }
    }
}

impl core::ops::AddAssign<u8> for Cell {
    #[inline(always)]
    fn add_assign(&mut self, rhs: u8) {
        self.0.add_assign(unsafe { core::mem::transmute::<_, i8>(rhs) })
    }
}

impl core::ops::AddAssign<&u8> for Cell {
    #[inline(always)]
    fn add_assign(&mut self, rhs: &u8) {
        self.0.add_assign(unsafe { core::mem::transmute::<_, &i8>(rhs) })
    }
}

impl PartialEq<i8> for Cell {
    #[inline(always)]
    fn eq(&self, other: &i8) -> bool {
        self.0.eq(other)
    }
}

/// Chunks of 64 cells (8x8)
///
/// TODO::
/// - make private, explose only a single top level `eval` function
/// - optimize for SIMD
#[derive(PartialEq)]
#[repr(transparent)]
pub struct Chunk ([[Cell; 8]; 8]);

impl core::fmt::Debug for Chunk {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl core::ops::Deref for Chunk {
    type Target = [[Cell; 8]; 8];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}

impl core::ops::DerefMut for Chunk {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}

impl Chunk {
    #[inline(always)]
    pub const fn from_array(src: [[i8; 8]; 8]) -> Self {
        // Safety:
        // - it's safe to transmute `i8` to `Cell` due to `Cell` being a 
        //   transparent wrapper of `i8`;
        // So it's safe to cast the whole array of `u8` to the array of `Cell`
        Self(unsafe { core::mem::transmute::<_, [[Cell; 8]; 8]>(src) }) 
    }

    /// Extract bit-encoded cells into [`Chunk`](a 2d array of [`Cell`]).
    #[inline(always)]
    pub fn extract(src: &u64) -> Self {
        let mut cells = [[0u8; 8]; 8];

        cells.iter_mut().zip(src.to_le_bytes().iter()).for_each(|(col, src)| {
            col.iter_mut().enumerate().for_each(|(i, cell)| {
                // According to the definition, "live" is negative and
                // "dead" is positive, so bitshift the target bit to the 
                // sign bit position and filter the rest.
                *cell = 128 & (src << i);
            })
        });

        // Safety:
        // - it's safe to transmute `u8` to `i8`;
        // - it's safe to transmute `i8` to `Cell` due to `Cell` being a 
        //   transparent wrapper of `i8`;
        // So it's safe to cast the whole array of `u8` to the array of `Cell`
        Self(unsafe { core::mem::transmute::<_, [[Cell; 8]; 8]>(cells) })
    }

    /// Evaluate a state of each cell in the chunk and bit-encode the chunk.
    ///
    /// This applies Conway's rules to each cell of the chunk, so make sure to 
    /// update surroundings (8 adjacent chunks + itself) before calling this.
    #[inline(always)]
    pub fn eval(&self) -> u64 {
        let mut bytes = [0u8; 8];

        bytes.iter_mut().zip(self.iter()).for_each(|(dst, row)| {
            row.iter().enumerate().for_each(|(i, cell)| {
                *dst = *dst | (cell.eval() << (7 - i));
            })
        });

        u64::from_ne_bytes(bytes)
    }

    #[inline(always)]
    pub fn add_north_west(&mut self, north_west: &u64) {
        #[cfg(target_endian = "big")]
        let bit = north_west & 1;

        #[cfg(target_endian = "little")]
        let bit = (north_west >> 56) & 1;

        self[0][0] += bit as u8;
    }

    #[inline(always)]
    pub fn add_north(&mut self, north: &u64) {
        // Due to LE encoding the bottom row is the last byte of `u64`
        let bits = explode_byte(north.to_le_bytes()[7]);

        self[0][0] += bits[0] + bits[1];
        self[0][7] += bits[6] + bits[7];

        self[0].iter_mut().skip(1).take(6)
            .zip(bits.windows(3))
            .for_each(|(dst, tri)| {
                tri.iter().for_each(|bit| *dst += bit);
            })
    }

    #[inline(always)]
    pub fn add_north_east(&mut self, north_east: &u64) {
        #[cfg(target_endian = "big")]
        let bit = (north_east >> 7) & 1;

        #[cfg(target_endian = "little")]
        let bit = (north_east >> 63) & 1;

        self[0][7] += bit as u8;
    }

    #[inline(always)]
    pub fn add_west(&mut self, west: &u64) {
        let mut bits = [0u8; 8];
        bits.iter_mut()
            .zip(west.to_le_bytes().iter())
            .for_each(|(dst, byte)| *dst = *byte & 1);

        self[0][0] += bits[0] + bits[1];
        self[7][0] += bits[6] + bits[7];

        self.iter_mut().skip(1).take(6)
            .zip(bits.windows(3))
            .for_each(|(row, tri)| {
                tri.iter().for_each(|bit| row[0] += bit);
            })
    }

    #[inline(always)]
    pub fn add_center(&mut self, center: &u64) {
        let mut cells = [[0u8; 8]; 8];

        cells.iter_mut().zip(center.to_le_bytes().iter()).for_each(|(row, byte)| {
            *row = explode_byte(*byte);
        });

        println!("{:0>64b}", center);
        println!("{:?}", cells);

        self[0][0] += cells[0][1] + cells[1][0] + cells[1][1];
        self[0][7] += cells[0][6] + cells[1][6] + cells[1][7];
        self[7][0] += cells[7][1] + cells[6][0] + cells[6][1];
        self[7][7] += cells[7][6] + cells[6][6] + cells[6][7];

        for i in 1..7 {
            self[0][i] += cells[0][i-1]               + cells[0][i+1]
                        + cells[1][i-1] + cells[1][i] + cells[1][i+1];

            self[7][i] += cells[6][i-1] + cells[6][i] + cells[6][i+1]
                        + cells[7][i-1]               + cells[7][i+1];

            self[i][0] += cells[i-1][0] + cells[i-1][1]
                                        + cells[i  ][1]
                        + cells[i+1][0] + cells[i+1][1];

            self[i][7] += cells[i-1][6] + cells[i-1][7]
                        + cells[i  ][6]
                        + cells[i+1][6] + cells[i+1][7];

            for j in 1..7 {
                self[i][j] += cells[i-1][j-1] + cells[i-1][j  ] + cells[i-1][j+1]
                            + cells[i  ][j-1]                   + cells[i  ][j+1]
                            + cells[i+1][j-1] + cells[i+1][j  ] + cells[i+1][j+1];
            }
        }
    }

    #[inline(always)]
    pub fn add_east(&mut self, east: &u64) {
        let mut bits = [0u8; 8];
        bits.iter_mut()
            .zip(east.to_le_bytes().iter())
            .for_each(|(dst, byte)| *dst = *byte & 1);

        self[0][7] += bits[0] + bits[1];
        self[7][7] += bits[6] + bits[7];

        self.iter_mut().skip(1).take(6)
            .zip(bits.windows(3))
            .for_each(|(row, tri)| {
                tri.iter().for_each(|bit| row[7] += bit);
            })
    }

    #[inline(always)]
    pub fn add_south_west(&mut self, south_west: &u64) {
        #[cfg(target_endian = "big")]
        let bit = (south_west >> 56) & 1;

        #[cfg(target_endian = "little")]
        let bit = south_west & 1;

        self[7][0] += bit as u8;
    }

    #[inline(always)]
    pub fn add_south(&mut self, south: &u64) {
        // Due to LE encoding the top row is the first byte of `u64`
        let bits = explode_byte(south.to_le_bytes()[0]);

        self[7][0] += bits[0] + bits[1];
        self[7][7] += bits[6] + bits[7];

        self[7].iter_mut().skip(1).take(6)
            .zip(bits.windows(3))
            .for_each(|(dst, tri)| {
                tri.iter().for_each(|bit| *dst += bit);
            })
    }

    #[inline(always)]
    pub fn add_south_east(&mut self, south_east: &u64) {
        #[cfg(target_endian = "big")]
        let bit = (south_east >> 63) & 1;

        #[cfg(target_endian = "little")]
        let bit = (south_east >> 7) & 1;

        self[7][7] += bit as u8;
    }
}

#[cfg(test)]
mod chunk {
    use super::Chunk;

    #[test]
    fn extract() {
        let c: u64 = 0b00000001_00000010_00000100_00001000_00010000_00100000_01000000_10000000;
        
        let cells: [[i8; 8]; 8] = [
            [-128, 0, 0, 0, 0, 0, 0, 0],
            [0, -128, 0, 0, 0, 0, 0, 0],
            [0, 0, -128, 0, 0, 0, 0, 0],
            [0, 0, 0, -128, 0, 0, 0, 0],
            [0, 0, 0, 0, -128, 0, 0, 0],
            [0, 0, 0, 0, 0, -128, 0, 0],
            [0, 0, 0, 0, 0, 0, -128, 0],
            [0, 0, 0, 0, 0, 0, 0, -128],
        ];

        assert_eq!(Chunk::extract(&c), Chunk::from_array(cells));
    }

    #[test]
    fn eval() {
        let chunk = Chunk::from_array([
            [-126,    0, -125,    0,    3, -128, -125, -122],
            [-128, -125,    0,    3,    0, -126,    0, -125],
            [   0,    0,    0,    0,    0,    0, -126, -125],
            [   0,    0,    0, -126,    0,    0,    0,    0],
            [   0,    0,    0,    0,    3,    0,    0,    0],
            [   3, -125,    0,    0,    0,    0,    0,    0],
            [-127, -126, -125, -124, -123, -122, -121, -120],
            [   1,    2,    3,    4,    5,    6,    7,    8],
        ]);
        
        let c: u64 = 0b00100000_01100000_11000000_00001000_00010000_00000011_01010101_10101010;

        assert_eq!(chunk.eval(), c);
    }

    #[test]
    fn add_north_west() {
        let mut chunk = Chunk::extract(&(1 << 7));

        chunk.add_north_west(&0b0);
        assert_eq!(chunk[0][0], -128);

        chunk.add_north_west(&(1 << 56));
        assert_eq!(chunk[0][0], -127);

        let mut chunk = Chunk::extract(&0);

        chunk.add_north_west(&0);
        assert_eq!(chunk[0][0], 0);

        chunk.add_north_west(&(1 << 56));
        assert_eq!(chunk[0][0], 1);
    }

    #[test]
    fn add_north() {
        let mut chunk = Chunk::extract(&0b10001000);

        chunk.add_north(&0);
        assert_eq!(chunk[0], [-128, 0, 0, 0, -128, 0, 0, 0]);

        chunk.add_north(&(0b01011100 << 56));
        assert_eq!(chunk[0], [-127, 1, 2, 2, -125, 2, 1, 0]);
    }

    #[test]
    fn add_north_east() {
        let mut chunk = Chunk::extract(&1);

        chunk.add_north_east(&0);
        assert_eq!(chunk[0][7], -128);

        chunk.add_north_east(&(1 << 63));
        assert_eq!(chunk[0][7], -127);
    }

    #[test]
    fn add_west() {
        let mut chunk = Chunk::from_array([
            [-128, 0, 0, 0, 0, 0, 0, 0],
            [0i8; 8],
            [0i8; 8],
            [0i8; 8],
            [-128, 0, 0, 0, 0, 0, 0, 0],
            [0i8; 8],
            [0i8; 8],
            [0i8; 8],
        ]);

        chunk.add_west(&0);
        assert_eq!(chunk[0][0], -128);
        assert_eq!(chunk[1][0], 0);
        assert_eq!(chunk[2][0], 0);
        assert_eq!(chunk[4][0], -128);

        chunk.add_west(&0b00000000_00000000_00000001_00000001_00000001_00000000_00000001_00000000);
        assert_eq!(chunk[0][0], -127);
        assert_eq!(chunk[1][0], 1);
        assert_eq!(chunk[2][0], 2);
        assert_eq!(chunk[3][0], 2);
        assert_eq!(chunk[4][0], -125);
        assert_eq!(chunk[5][0], 2);
        assert_eq!(chunk[6][0], 1);
        assert_eq!(chunk[7][0], 0);
    }

    #[test]
    fn add_center() {
        let pack: u64 = 0b00100000_01100000_11000000_00001000_00010000_00000011_01010101_10101010;
        let mut chunk = Chunk::extract(&pack);

        // @.@.@.@.
        // .@.@.@.@
        // ......@@
        // ...@....
        // ....@...
        // @@......
        // .@@.....
        // ..@.....
        
        let res: [[i8; 8]; 8] = [
            [-127,    3, -126,    3, -126,    3, -126,    2],
            [   2, -126,    3, -126,    3, -125,    5, -125],
            [   1,    1,    3,    2,    3,    2, -125, -126],
            [   0,    0,    1, -127,    2,    2,    2,    2],
            [   2,    2,    2,    2, -127,    1,    0,    0],
            [-126, -125,    3,    2,    1,    1,    0,    0],
            [   3, -124, -125,    2,    0,    0,    0,    0],
            [   1,    3, -126,    2,    0,    0,    0,    0],
        ];

        chunk.add_center(&pack);
        assert_eq!(chunk, Chunk::from_array(res));
    }

    #[test]
    fn add_east() {
        let mut chunk = Chunk::from_array([ 
            [0, 0, 0, 0, 0, 0, 0, -128],
            [0i8; 8],
            [0i8; 8],
            [0i8; 8],
            [0, 0, 0, 0, 0, 0, 0, -128],
            [0i8; 8],
            [0i8; 8],
            [0i8; 8],
        ]);

        chunk.add_east(&0);
        assert_eq!(chunk[0][7], -128);
        assert_eq!(chunk[4][7], -128);
        assert_eq!(chunk[1][7], 0);
        assert_eq!(chunk[5][7], 0);
        assert_eq!(chunk[7][7], 0);

        chunk.add_east(&0b00000000_00000000_00000001_00000001_00000001_00000000_00000001_00000000);
        for (i, res) in [-127, 1, 2, 2, -125, 2, 1, 0].iter().enumerate() {
            assert_eq!(chunk[i][7], *res);
        }
    }

    #[test]
    fn add_south_west() {
        let mut chunk = Chunk::extract(&(1 << 63));

        chunk.add_south_west(&0);
        assert_eq!(chunk[7][0], -128);

        chunk.add_south_west(&1);
        assert_eq!(chunk[7][0], -127);


        let mut chunk = Chunk::extract(&0);

        chunk.add_south_west(&1);
        assert_eq!(chunk[7][0], 1);
    }

    #[test]
    fn add_south() {
        let mut chunk = Chunk::extract(&(0b10001000 << 56));

        chunk.add_south(&0b01011100);
        assert_eq!(chunk[7], [-127, 1, 2, 2, -125, 2, 1, 0]);
    }

    #[test]
    fn add_south_east() {
        let mut chunk = Chunk::extract(&(1 << 56));

        chunk.add_south_east(&0);
        assert_eq!(chunk[7][7], -128);

        chunk.add_south_east(&(1 << 7));
        assert_eq!(chunk[7][7], -127);
    }
}

#[inline(always)]
pub const fn explode_byte(byte: u8) -> [u8; 8] {
    [
        byte >> 7,
        (byte >> 6) & 1,
        (byte >> 5) & 1,
        (byte >> 4) & 1,
        (byte >> 3) & 1,
        (byte >> 2) & 1,
        (byte >> 1) & 1,
        byte & 1,
    ]
}
