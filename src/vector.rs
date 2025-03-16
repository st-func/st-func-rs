use std::ops::{Add, Index, IndexMut, Sub};

/// ベクトルを表す構造体
#[derive(Debug, Clone)]
pub struct Vector<T> {
    size: usize,
    data: Vec<T>,
}

impl<T: Default + Clone> Vector<T> {
    /// 新しいベクトルを作成する
    ///
    /// # 引数
    ///
    /// * `size` - ベクトルのサイズ
    ///
    /// # 戻り値
    ///
    /// 新しいベクトル
    pub fn new(size: usize) -> Self {
        let data = vec![T::default(); size];
        Vector { size, data }
    }

    /// ベクタからベクトルを作成する
    ///
    /// # 引数
    ///
    /// * `data` - ベクトルのデータ
    ///
    /// # 戻り値
    ///
    /// 新しいベクトル
    pub fn from_vec(data: Vec<T>) -> Self {
        let size = data.len();
        Vector { size, data }
    }
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;

    /// ベクトルの要素を取得する
    ///
    /// # 引数
    ///
    /// * `index` - インデックス
    ///
    /// # 戻り値
    ///
    /// 指定された位置の要素への参照
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Vector<T> {
    /// ベクトルの要素を変更する
    ///
    /// # 引数
    ///
    /// * `index` - インデックス
    ///
    /// # 戻り値
    ///
    /// 指定された位置の要素への可変参照
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> Add for Vector<T>
where
    T: Add<Output = T> + Default + Copy,
{
    type Output = Vector<T>;

    /// 2つのベクトルを加算する
    ///
    /// # 引数
    ///
    /// * `other` - 加算するベクトル
    ///
    /// # 戻り値
    ///
    /// 加算結果のベクトル
    ///
    /// # パニック
    ///
    /// ベクトルのサイズが一致しない場合にパニックする
    fn add(self, other: Vector<T>) -> Vector<T> {
        assert!(self.size == other.size);
        let mut result = Vector::new(self.size);
        for i in 0..self.size {
            result[i] = self[i] + other[i];
        }
        result
    }
}

impl<T> Sub for Vector<T>
where
    T: Sub<Output = T> + Default + Copy,
{
    type Output = Vector<T>;

    /// 2つのベクトルを減算する
    ///
    /// # 引数
    ///
    /// * `other` - 減算するベクトル
    ///
    /// # 戻り値
    ///
    /// 減算結果のベクトル
    ///
    /// # パニック
    ///
    /// ベクトルのサイズが一致しない場合にパニックする
    fn sub(self, other: Vector<T>) -> Vector<T> {
        assert!(self.size == other.size);
        let mut result = Vector::new(self.size);
        for i in 0..self.size {
            result[i] = self[i] - other[i];
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let v: Vector<f64> = Vector::new(3);
        assert_eq!(v[0], 0.0);
        assert_eq!(v[2], 0.0);
    }

    #[test]
    fn test_vector_addition() {
        let v1 = Vector::from_vec(vec![1.0, 2.0, 3.0]);
        let v2 = Vector::from_vec(vec![4.0, 5.0, 6.0]);
        let v3 = v1 + v2;
        assert_eq!(v3[0], 5.0);
        assert_eq!(v3[1], 7.0);
        assert_eq!(v3[2], 9.0);
    }

    #[test]
    fn test_vector_subtraction() {
        let v1 = Vector::from_vec(vec![4.0, 5.0, 6.0]);
        let v2 = Vector::from_vec(vec![1.0, 2.0, 3.0]);
        let v3 = v1 - v2;
        assert_eq!(v3[0], 3.0);
        assert_eq!(v3[1], 3.0);
        assert_eq!(v3[2], 3.0);
    }

    #[test]
    fn test_vector_indexing() {
        let mut v: Vector<f64> = Vector::new(3);
        v[0] = 1.0;
        v[1] = 2.0;
        v[2] = 3.0;
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
    }
}
