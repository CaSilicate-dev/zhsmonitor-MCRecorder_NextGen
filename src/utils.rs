pub fn advanced_round(value: f64, decimal: i32) -> f64 {
    let factor = 10f64.powi(decimal);
    ((value * factor).round()) / factor
}
