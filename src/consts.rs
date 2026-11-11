pub const MAX_P : usize = 15;
pub const FULL_LEN: usize = MAX_P + 3;
pub const OMEGA: i64 = 19; //4 <= Omega <=19
pub const D: i64 = 16;
pub const R: i64 = 40;
//supporting only unsigned samples
pub const S_MIN: i64 = 0;
pub const S_MAX: i64 = (1<<D) - 1;
pub const S_MID: i64 = 1<<(D-1);
pub const V_MIN: i64 = 3;
pub const V_MAX: i64 = 6;
pub const T_INC: i64 = 1<<6;
pub const W_MIN: i64 = -(1<<(OMEGA+2));
pub const W_MAX: i64 = (1<<(OMEGA+2))-1;