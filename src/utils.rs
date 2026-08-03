pub fn mod_star(x: i64, r: usize) -> i64 {
    let modulus = 1i64 << r;
    let half = 1i64 << (r - 1);
    ((x + half).rem_euclid(modulus)) - half
}