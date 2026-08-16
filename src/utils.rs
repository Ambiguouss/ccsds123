use crate::consts::MAX_P;
pub fn mod_star(x: i64, r: i64) -> i64 {
    let modulus = 1i64 << r;
    let half = 1i64 << (r - 1);
    ((x + half).rem_euclid(modulus)) - half
}
pub fn sgn_plus(x:i64) -> i64 {
    if x>=0 { return 1;}
    return -1;
}
pub fn shift_right(x:i64,y:i64) -> i64 {
    if y >= 0{
        return x>>y;
    }
    return x<<(-y);
}
pub fn dot_product(weights: &[i64; MAX_P], values: &[i64; MAX_P]) -> i64 {
    weights.iter()
        .zip(values.iter())
        .map(|(w, v)| *w* *v)
        .sum()
}