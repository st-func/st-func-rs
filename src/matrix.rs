use std::ops::{Add, Index, IndexMut};

/// 行列を表す構造体
#[derive(Debug, Clone)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<f64>>,
}

impl Matrix {
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
        let data = vec![vec![0.0; cols]; rows];
        Matrix { rows, cols, data }
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
    pub fn from_vec(data: Vec<Vec<f64>>) -> Self {
        let rows = data.len();
        let cols = data[0].len();
        Matrix { rows, cols, data }
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

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

impl IndexMut<(usize, usize)> for Matrix {
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

impl Add for Matrix {
    type Output = Matrix;

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
    fn add(self, other: Matrix) -> Matrix {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_creation() {
        let m = Matrix::new(2, 3);
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
        let mut m = Matrix::new(2, 2);
        m[(0, 0)] = 1.0;
        m[(0, 1)] = 2.0;
        m[(1, 0)] = 3.0;
        m[(1, 1)] = 4.0;
        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(0, 1)], 2.0);
        assert_eq!(m[(1, 0)], 3.0);
        assert_eq!(m[(1, 1)], 4.0);
    }
}
