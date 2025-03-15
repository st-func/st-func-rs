use std::ops::{Add, Index, IndexMut, Mul, Sub};

/// 行列を表す構造体
#[derive(Debug, Clone)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<Vec<T>>,
}

impl<T: Default + Clone> Matrix<T> {
    /// 新しい行列を作成する
    ///
    /// # 引数
    ///
    /// * `rows` - 行数
    /// * `cols` - 列数
    ///
    /// # 戻り値
    ///
    /// 新しい行列
    pub fn new(rows: usize, cols: usize) -> Self {
        let data = vec![vec![T::default(); cols]; rows];
        Matrix { rows, cols, data }
    }

    /// 正方行列を作成する
    ///
    /// # 引数
    ///
    /// * `size` - 行列のサイズ
    ///
    /// # 戻り値
    ///
    /// 新しい正方行列
    pub fn new_square(size: usize) -> Self {
        Self::new(size, size)
    }

    /// ベクタから行列を作成する
    ///
    /// # 引数
    ///
    /// * `data` - 行列のデータ
    ///
    /// # 戻り値
    ///
    /// 新しい行列
    pub fn from_vec(data: Vec<Vec<T>>) -> Self {
        let rows = data.len();
        let cols = data[0].len();
        Matrix { rows, cols, data }
    }

    /// 配列から行列を作成する
    ///
    /// # 引数
    ///
    /// * `data` - 行列のデータ
    ///
    /// # 戻り値
    ///
    /// 新しい行列
    pub fn from_array<const R: usize, const C: usize>(data: [[T; C]; R]) -> Self {
        let data = data.iter().map(|row| row.to_vec()).collect();
        Matrix {
            rows: R,
            cols: C,
            data,
        }
    }
}

impl<T: Default + Copy> Matrix<T> {
    /// 行列を転置する
    ///
    /// # 戻り値
    ///
    /// 転置された行列
    pub fn transpose(&self) -> Self {
        let mut transposed_data = vec![vec![T::default(); self.rows]; self.cols];
        for i in 0..self.rows {
            for j in 0..self.cols {
                transposed_data[j][i] = self.data[i][j];
            }
        }
        Matrix {
            rows: self.cols,
            cols: self.rows,
            data: transposed_data,
        }
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    /// 行列の要素を取得する
    ///
    /// # 引数
    ///
    /// * `index` - 行と列のインデックス
    ///
    /// # 戻り値
    ///
    /// 指定された位置の要素への参照
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    /// 行列の要素を変更する
    ///
    /// # 引数
    ///
    /// * `index` - 行と列のインデックス
    ///
    /// # 戻り値
    ///
    /// 指定された位置の要素への可変参照
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}

impl<T> Add for Matrix<T>
where
    T: Add<Output = T> + Default + Copy,
{
    type Output = Matrix<T>;

    /// 2つの行列を加算する
    ///
    /// # 引数
    ///
    /// * `other` - 加算する行列
    ///
    /// # 戻り値
    ///
    /// 加算結果の行列
    ///
    /// # パニック
    ///
    /// 行列のサイズが一致しない場合にパニックする
    fn add(self, other: Matrix<T>) -> Matrix<T> {
        assert!(self.rows == other.rows && self.cols == other.cols);
        let mut result = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result[(i, j)] = self[(i, j)] + other[(i, j)];
            }
        }
        result
    }
}

impl<T> Sub for Matrix<T>
where
    T: Sub<Output = T> + Default + Copy,
{
    type Output = Matrix<T>;

    /// 2つの行列を減算する
    ///
    /// # 引数
    ///
    /// * `other` - 減算する行列
    ///
    /// # 戻り値
    ///
    /// 減算結果の行列
    ///
    /// # パニック
    ///
    /// 行列のサイズが一致しない場合にパニックする
    fn sub(self, other: Matrix<T>) -> Matrix<T> {
        assert!(self.rows == other.rows && self.cols == other.cols);
        let mut result = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result[(i, j)] = self[(i, j)] - other[(i, j)];
            }
        }
        result
    }
}

impl<T> Mul for Matrix<T>
where
    T: Mul<Output = T> + Add<Output = T> + Default + Copy,
{
    type Output = Matrix<T>;

    /// 2つの行列を掛け算する
    ///
    /// # 引数
    ///
    /// * `other` - 掛け算する行列
    ///
    /// # 戻り値
    ///
    /// 掛け算結果の行列
    ///
    /// # パニック
    ///
    /// 行列のサイズが適切でない場合にパニックする
    fn mul(self, other: Matrix<T>) -> Matrix<T> {
        assert!(self.cols == other.rows);
        let mut result = Matrix::new(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    result[(i, j)] = result[(i, j)] + (self[(i, k)] * other[(k, j)]);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_creation() {
        let m: Matrix<f64> = Matrix::new(2, 3);
        assert_eq!(m[(0, 0)], 0.0);
        assert_eq!(m[(1, 2)], 0.0);
    }

    #[test]
    fn test_matrix_addition() {
        let m1 = Matrix::from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let m2 = Matrix::from_vec(vec![vec![5.0, 6.0], vec![7.0, 8.0]]);
        let m3 = m1 + m2;
        assert_eq!(m3[(0, 0)], 6.0);
        assert_eq!(m3[(1, 1)], 12.0);
    }

    #[test]
    fn test_matrix_indexing() {
        let mut m: Matrix<f64> = Matrix::new(2, 2);
        m[(0, 0)] = 1.0;
        m[(0, 1)] = 2.0;
        m[(1, 0)] = 3.0;
        m[(1, 1)] = 4.0;
        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(0, 1)], 2.0);
        assert_eq!(m[(1, 0)], 3.0);
        assert_eq!(m[(1, 1)], 4.0);
    }

    #[test]
    fn test_matrix_subtraction() {
        let m1 = Matrix::from_vec(vec![vec![5.0, 6.0], vec![7.0, 8.0]]);
        let m2 = Matrix::from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let m3 = m1 - m2;
        assert_eq!(m3[(0, 0)], 4.0);
        assert_eq!(m3[(1, 1)], 4.0);
    }

    #[test]
    fn test_matrix_multiplication() {
        let m1 = Matrix::from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let m2 = Matrix::from_vec(vec![vec![2.0, 0.0], vec![1.0, 2.0]]);
        let m3 = m1 * m2;
        assert_eq!(m3[(0, 0)], 4.0);
        assert_eq!(m3[(0, 1)], 4.0);
        assert_eq!(m3[(1, 0)], 10.0);
        assert_eq!(m3[(1, 1)], 8.0);
    }

    #[test]
    fn test_matrix_transpose() {
        let m = Matrix::from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let mt = m.transpose();
        assert_eq!(mt[(0, 0)], 1.0);
        assert_eq!(mt[(0, 1)], 3.0);
        assert_eq!(mt[(1, 0)], 2.0);
        assert_eq!(mt[(1, 1)], 4.0);
    }

    #[test]
    fn test_matrix_from_array() {
        let data = [[1.0, 2.0], [3.0, 4.0]];
        let m = Matrix::from_array(data);
        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(0, 1)], 2.0);
        assert_eq!(m[(1, 0)], 3.0);
        assert_eq!(m[(1, 1)], 4.0);
    }
}
