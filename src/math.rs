use std::ops::{Index, IndexMut, Add, Sub, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<const N: usize> {
    data: [[f32; N]; N],
}

impl<const N: usize> Default for Matrix<N> {
    fn default() -> Self {
        Self { data: [[0.0; N]; N] }
    }
}

impl<const N: usize> Matrix<N> {
    pub fn identity() -> Self {
        let mut data = [[0.0; N]; N];
        for i in 0..N {
            data[i][i] = 1.0;
        }
        Self { data }
    }
}

impl<const N: usize> From<[[f32; N]; N]> for Matrix<N> {
    fn from(data: [[f32; N]; N]) -> Self {
        Self { data }
    }
}

impl<const N: usize> Index<(usize, usize)> for Matrix<N> {
    type Output = f32;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row][col]
    }
}

impl<const N: usize> IndexMut<(usize, usize)> for Matrix<N> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[row][col]
    }
}

macro_rules! impl_element_binary_op {
    ($trait:ident, $method:ident) => {
        impl<const N: usize> $trait<Matrix<N>> for Matrix<N> {
            type Output = Matrix<N>;

            fn $method(self, rhs: Matrix<N>) -> Self::Output {
                let mut result = Self::default();
                for i in 0..N {
                    for j in 0..N {
                        result[(i, j)] = $trait::$method(self[(i, j)], rhs[(i, j)]);
                    }
                }
                result
            }
        }
    };
}

impl_element_binary_op!(Add, add);
impl_element_binary_op!(Sub, sub);

impl<const N: usize> Mul<Matrix<N>> for Matrix<N> {
    type Output = Matrix<N>;

    fn mul(self, rhs: Matrix<N>) -> Self::Output {
        let mut result = Self::default();

        for i in 0..N {
            for j in 0..N {
                for k in 0..N {
                    result[(j, i)] += self[(j, k)] * rhs[(k, i)];
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(Matrix::<3>::default() + Matrix::default(), Matrix::default());

        let m1 = Matrix::from([
            [1.0, 2.0],
            [3.0, 4.0],
        ]);
        let m2 = Matrix::from([
            [5.0, 6.0],
            [7.0, 8.0],
        ]);

        assert_eq!(m1 + m2, Matrix::from([
            [6.0, 8.0],
            [10.0, 12.0],
        ]));
    }

    #[test]
    fn sub() {
        assert_eq!(Matrix::<3>::default() - Matrix::default(), Matrix::default());

        let m1 = Matrix::from([
            [1.0, 2.0],
            [3.0, 4.0],
        ]);
        let m2 = Matrix::from([
            [5.0, 6.0],
            [7.0, 8.0],
        ]);

        assert_eq!(m1 - m2, Matrix::from([
            [-4.0, -4.0],
            [-4.0, -4.0],
        ]));
    }

    #[test]
    fn mul() {
        assert_eq!(Matrix::<3>::default() * Matrix::default(), Matrix::default());
        assert_eq!(Matrix::<3>::identity() * Matrix::identity(), Matrix::identity());

        let m1 = Matrix::from([
            [1.0, 2.0],
            [3.0, 4.0],
        ]);
        let m2 = Matrix::from([
            [5.0, 6.0],
            [7.0, 8.0],
        ]);

        assert_eq!(m1 * m2, Matrix::from([
            [19.0, 22.0],
            [43.0, 50.0],
        ]));
    }
}
