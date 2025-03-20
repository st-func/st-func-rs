use crate::vector::Vector;

/// 1質点系の応答を表す構造体
///
/// # フィールド
///
/// * `absolute_acceleration` - 絶対応答加速度
/// * `relative_velocity` - 相対応答速度
/// * `relative_displacement` - 相対応答変位
pub struct SdofResponse {
    pub absolute_acceleration: Vector<f64>,
    pub relative_velocity: Vector<f64>,
    pub relative_displacement: Vector<f64>,
}

/// Nigam-Jennings法を用いて1質点系の応答を計算する関数
///
/// # 引数
///
/// * `y0_ddot` - 地動加速度
/// * `delta_t` - 時間刻み
/// * `omega` - 固有円振動数
/// * `h` - 減衰定数
///
/// # 戻り値
///
/// 1質点系の応答を表す `SdofResponse` 構造体
pub fn nigam_jennings(y0_ddot: &Vector<f64>, delta_t: f64, omega: f64, h: f64) -> SdofResponse {
    let n: usize = y0_ddot.len();

    let mut y: Vector<f64> = Vector::new(n); // 変位時刻歴
    let mut y_dot: Vector<f64> = Vector::new(n); // 速度時刻歴
    let mut y_y0_ddot: Vector<f64> = Vector::new(n); // 加速度時刻歴

    let mut y_pre: f64 = 0.0; // 前ステップの変位
    let mut y_dot_pre: f64 = 0.0; // 前ステップの速度
    let mut y0_ddot_pre: f64 = y0_ddot[0]; // 前ステップの地動加速度

    let omega_dash: f64 = (1.0 - h * h).sqrt() * omega;
    let a11: f64 = (-h * omega * delta_t).exp()
        * (h / (1.0 - h * h).sqrt() * (omega_dash * delta_t).sin() + (omega_dash * delta_t).cos());
    let a12: f64 = (-h * omega * delta_t).exp() / omega_dash * (omega_dash * delta_t).sin();
    let a21: f64 =
        -omega / (1.0 - h * h).sqrt() * (-h * omega * delta_t).exp() * (omega_dash * delta_t).sin();
    let a22: f64 = (-h * omega * delta_t).exp()
        * ((omega_dash * delta_t).cos() - h / (1.0 - h * h).sqrt() * (omega_dash * delta_t).sin());
    let b11: f64 = (-h * omega * delta_t).exp()
        * (((2.0 * h * h - 1.0) / (omega * omega * delta_t) + h / omega)
            * (omega_dash * delta_t).sin()
            / omega_dash
            + (2.0 * h / (omega * omega * omega * delta_t)
                + 1.0 / (omega * omega) * (omega_dash * delta_t).cos()))
        - 2.0 * h / (omega * omega * omega * delta_t);
    let b12: f64 = -(-h * omega * delta_t).exp()
        * ((2.0 * h * h - 1.0) / (omega * omega * delta_t) * (omega_dash * delta_t).sin()
            / omega_dash
            + 2.0 * h / (omega * omega * omega * delta_t) * (omega_dash * delta_t).cos())
        - 1.0 / (omega * omega)
        + 2.0 * h / (omega * omega * omega * delta_t);
    let b21: f64 = (-h * omega * delta_t).exp()
        * (((2.0 * h * h - 1.0) / (omega * omega * delta_t) + h / omega)
            * ((omega_dash * delta_t).cos()
                - h / (1.0 - h * h).sqrt() * (omega_dash * delta_t).sin())
            - (2.0 * h / (omega * omega * omega * delta_t) + 1.0 / (omega * omega))
                * (omega_dash * (omega_dash * delta_t).sin()
                    + h * omega * (omega_dash * delta_t).cos()))
        + 1.0 / (omega * omega * delta_t);
    let b22: f64 = -(-h * omega * delta_t).exp()
        * ((2.0 * h * h - 1.0) / (omega * omega * delta_t)
            * ((omega_dash * delta_t).cos()
                - h / (1.0 - h * h).sqrt() * (omega_dash * delta_t).sin())
            - 2.0 * h / (omega * omega * omega * delta_t)
                * (omega_dash * (omega_dash * delta_t).sin()
                    + h * omega * (omega_dash * delta_t).cos()))
        - 1.0 / (omega * omega * delta_t);

    for i in 1..n {
        let y0_ddot_cur: f64 = y0_ddot[i];
        let y_cur: f64 = a11 * y_pre + a12 * y_dot_pre + b11 * y0_ddot_pre + b12 * y0_ddot_cur;
        let y_dot_cur: f64 = a21 * y_pre + a22 * y_dot_pre + b21 * y0_ddot_pre + b22 * y0_ddot_cur;
        let y_y0_ddot_cur: f64 = 2.0 * h * omega * y_dot_cur + omega * omega * y_cur;

        y[i] = y_cur;
        y_dot[i] = y_dot_cur;
        y_y0_ddot[i] = y_y0_ddot_cur;

        y_pre = y_cur;
        y_dot_pre = y_dot_cur;
        y0_ddot_pre = y0_ddot_cur;
    }

    SdofResponse {
        absolute_acceleration: y_y0_ddot,
        relative_velocity: y_dot,
        relative_displacement: y,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::f64::consts::PI;
    use std::fs::File;
    use std::io::{BufWriter, Write};

    #[test]

    /// Nigam-Jennings法を用いたステップ荷重応答のテスト
    ///
    /// このテストでは、Nigam-Jennings法を用いて1質点系のステップ荷重に対する応答を計算し、
    /// 理論式で算出した相対応答変位と実際の計算結果を比較します。
    /// 許容誤差内で一致することを確認します。
    ///
    /// 環境変数 `WRITE_CSV` が `test_nigam_jennings_step_load` に設定されている場合、
    /// テスト結果をCSVファイルに書き出します。下記のようなコマンドで実行できます。
    /// WRITE_CSV=test_nigam_jennings_step_load cargo test
    fn test_nigam_jennings_step_load() {
        let alpha: f64 = 3.0;
        let t: f64 = 0.1;
        let omega: f64 = 2.0 * PI / t;
        let h: f64 = 0.05;
        let omega_dash: f64 = (1.0 - h * h).sqrt() * omega;
        let delta_t: f64 = 0.01;
        let mut y0_ddot: Vector<f64> = Vector::<f64>::new(100);
        for i in 0..y0_ddot.len() {
            y0_ddot[i] = -alpha;
        }
        let response: SdofResponse = nigam_jennings(&y0_ddot, delta_t, omega, h);
        let tolerance: f64 = 1e-4; // 許容誤差

        // 環境変数をチェックしてCSVファイルに書き出すかどうかを決定
        let test_name = "test_nigam_jennings_step_load";
        let write_csv = env::var("WRITE_CSV").map_or(false, |val| val == test_name);
        let mut writer: Option<BufWriter<File>> = None;
        if write_csv {
            let output_dir = "test_output"; // テスト出力用のディレクトリ
            std::fs::create_dir_all(output_dir).expect("ディレクトリ作成に失敗しました");
            let file_path = format!("{}/{}.csv", output_dir, test_name);
            let file = File::create(file_path).expect("ファイル作成に失敗しました");
            writer = Some(BufWriter::new(file));
            writeln!(writer.as_mut().unwrap(), "index,expected,actual")
                .expect("ヘッダーの書き込みに失敗しました");
        }

        for i in 0..response.relative_displacement.len() {
            let t: f64 = i as f64 * delta_t;
            let expected: f64 = alpha / (omega * omega)
                * (1.0
                    - (-h * omega * t).exp()
                        * ((omega_dash * t).cos()
                            + h / (1.0 - h * h).sqrt() * (omega_dash * t).sin()));
            assert!(
                (expected - response.relative_displacement[i]).abs() < tolerance,
                "index: {}, expected: {}, actual: {}",
                i,
                expected,
                response.relative_displacement[i]
            );
            // CSVファイルにデータを書き込む
            if let Some(writer) = writer.as_mut() {
                writeln!(
                    writer,
                    "{},{},{}",
                    i, expected, response.relative_displacement[i]
                )
                .expect("データの書き込みに失敗しました");
            }
        }
    }
}
