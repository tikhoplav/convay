use crate::rand::fill;

pub type State<const N: usize> = [[u8; N]; N];

/// Create a new empty NxN state
pub fn new<const N: usize>() -> State<N> {
    [[0u8; N]; N]
}

/// Create a new NxN state with randomly seeded cells
#[cfg(feature = "rand")]
pub fn random<const N: usize>(seed: &[u8]) -> State<N> {
    let mut state = new();
    fill(&mut state, seed);
    state
}

/// Calculate next state from the previous one
pub fn tick<const N: usize>(prev: &State<N>, next: &mut State<N>) {
    for y in 0..N {
        for x in 0..N {
            let x0 = match x.checked_sub(1) {
                Some(x0) => x0,
                None => N - 1,
            };

            let y0 = match y.checked_sub(1) {
                Some(y0) => y0,
                None => N - 1,
            };

            let x1 = (x + 1) % N;
            let y1 = (y + 1) % N;

            let n = prev[y0][x0]
                + prev[y0][x]
                + prev[y0][x1]
                + prev[y][x0]
                + prev[y][x1]
                + prev[y1][x0]
                + prev[y1][x]
                + prev[y1][x1];

            next[y][x] = match prev[y][x] > 0 {
                true => (n > 1 && n < 4) as u8,
                false => (n == 3) as u8,
            };
        }
    }
}

#[cfg(test)]
mod test {
    use super::{new, tick, State};

    #[test]
    fn glider_test() {
        let prev: State<8> = [
            [0, 0, 1, 0, 0, 0, 0, 0],
            [1, 0, 1, 0, 0, 0, 0, 0],
            [0, 1, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
        ];

        let mut next: State<8> = new();
        tick(&prev, &mut next);

        assert_eq!(
            [
                [0, 1, 0, 0, 0, 0, 0, 0],
                [0, 0, 1, 1, 0, 0, 0, 0],
                [0, 1, 1, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
            ],
            next
        );

        let prev = next;
        let mut next: State<8> = new();
        tick(&prev, &mut next);

        assert_eq!(
            [
                [0, 0, 1, 0, 0, 0, 0, 0],
                [0, 0, 0, 1, 0, 0, 0, 0],
                [0, 1, 1, 1, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
            ],
            next
        );

        let prev = next;
        let mut next: State<8> = new();
        tick(&prev, &mut next);

        assert_eq!(
            [
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 1, 0, 1, 0, 0, 0, 0],
                [0, 0, 1, 1, 0, 0, 0, 0],
                [0, 0, 1, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
            ],
            next
        );

        let prev = next;
        let mut next: State<8> = new();
        tick(&prev, &mut next);

        assert_eq!(
            [
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 1, 0, 0, 0, 0],
                [0, 1, 0, 1, 0, 0, 0, 0],
                [0, 0, 1, 1, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
            ],
            next
        );
    }
}
