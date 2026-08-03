pub const MAX_P : usize = 15;
pub const FULL_LEN: usize = MAX_P + 3;
pub const OMEGA: usize = 19; //4 <= Omega <=19
pub const D: usize = 16;
pub const R: usize = 40;
//supporting only unsigned samples
pub const S_MIN: i64 = 0;
pub const S_MAX: i64 = 1<<D - 1;
pub const S_MID: i64 = 1<<(D-1);