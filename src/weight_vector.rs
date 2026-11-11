use crate::consts::{D,MAX_P,FULL_LEN,OMEGA,V_MIN,V_MAX,T_INC,W_MIN,W_MAX};
use crate::utils::{sgn_plus,shift_right};
use crate::hyperspectral_image::HyperspectralImage;
use crate::local_difference_vector::{CentralLocalDifferenceVectorReduced};
pub struct WeightVectorFull{
    pub weights : [i64;FULL_LEN],
}


pub struct WeightVectorReduced{
    pub weights : [i64;MAX_P],
}

impl WeightVectorReduced{
    pub fn new_default() -> Self{
        let val = 7*(1<<(OMEGA-3));
        let weights = core::array::from_fn(|i| {
            let shift = 3 * i;
            if shift >= 32 { 0 } else { val >> shift }
        });
        WeightVectorReduced{ weights }
    }
    pub fn update(&mut self,img: &HyperspectralImage,u: &CentralLocalDifferenceVectorReduced, s: i64, s_pred: i64,x:usize,y:usize,_z:usize){
        let e = 2*s-s_pred;
        let t = (x+img.x_size*y) as i64;
        let n_x = img.x_size as i64;
        let ro_basic = V_MIN + (t-n_x).div_euclid(T_INC);
        let ro = ro_basic.clamp(V_MIN,V_MAX)+D-OMEGA;
        for (i, v) in self.weights.iter_mut().enumerate() {
            let val = *v as i64+((shift_right(sgn_plus(e)*(u.get(i) as i64),ro as i64)+1)>>1); 
            *v=val.clamp(W_MIN,W_MAX) as i64;
        }
    }
}

impl WeightVectorFull{
    pub fn new_default() -> Self{
        let mut weights = [0i64; FULL_LEN];
        let bands: [i64; MAX_P] = WeightVectorReduced::new_default().weights;
        for (i, v) in bands.iter().enumerate() {
            weights[i + 3] = *v;
        }
        WeightVectorFull { weights }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::hyperspectral_image::SampleOrder;
    use crate::consts::S_MAX;
    use crate::local_difference_vector::CentralLocalDifferenceVectorReduced; // adjust path to your actual module

    fn test_u(values: [i64; MAX_P]) -> CentralLocalDifferenceVectorReduced {
        CentralLocalDifferenceVectorReduced::new_for_test(values)
    }
    #[test]
    fn reduced_first_weight_matches_formula() {
        let w = WeightVectorReduced::new_default();
        // ω^(1)(1) = (7/8) * 2^OMEGA
        let expected_first = 7 * (1 << (OMEGA - 3));
        assert_eq!(w.weights[0], expected_first);
    }

    #[test]
    fn reduced_each_subsequent_weight_is_prior_divided_by_8() {
        let w = WeightVectorReduced::new_default();
        for i in 1..MAX_P {
            assert_eq!(w.weights[i], w.weights[i - 1] / 8);
        }
    }

    #[test]
    fn reduced_weights_are_decreasing() {
        // sanity check on the shape of the sequence, independent of the exact formula
        let w = WeightVectorReduced::new_default();
        for i in 1..MAX_P {
            assert!(w.weights[i] <= w.weights[i - 1]);
        }
    }

    #[test]
    fn full_directional_weights_are_zero() {
        let w = WeightVectorFull::new_default();
        // spec 4.6.3.2.2: ω^N(1) = ω^W(1) = ω^NW(1) = 0
        assert_eq!(w.weights[0], 0);
        assert_eq!(w.weights[1], 0);
        assert_eq!(w.weights[2], 0);
    }

    #[test]
    fn full_band_weights_match_reduced_exactly() {
        let full = WeightVectorFull::new_default();
        let reduced = WeightVectorReduced::new_default();
        for i in 0..MAX_P {
            assert_eq!(full.weights[i + 3], reduced.weights[i]);
        }
    }

    #[test]
    fn full_and_reduced_are_distinct_types() {
        // compile-time property, not really a runtime assertion —
        // documents that these can't be accidentally interchanged
        let _full: WeightVectorFull = WeightVectorFull::new_default();
        let _reduced: WeightVectorReduced = WeightVectorReduced::new_default();
    }

    fn test_image() -> HyperspectralImage {
        HyperspectralImage {
            data: vec![1,2,3,4,5,6,7,8,9],
            x_size: 3, y_size: 3, z_size: 1,
            order: SampleOrder::BIL,
        }
    }

    #[test]
    fn first_row_does_not_panic() {
        // t < N_x for the entire first row — this is exactly the case that used to underflow
        let img = test_image();
        let mut w = WeightVectorReduced::new_default();
        let u = test_u([0; MAX_P]);

        // should not panic
        w.update(&img, &u, 5, 3, 0, 0, 0);
        w.update(&img, &u, 5, 3, 1, 0, 0);
        w.update(&img, &u, 5, 3, 2, 0, 0);
    }

    #[test]
    fn zero_error_and_zero_u_leaves_weights_near_unchanged() {
        // e=0 means sgn_plus(e) is still +1 (not 0, since sgn_plus never returns 0),
        // but u=0 everywhere means the delta term itself is 0 for every weight
        let img = test_image();
        let mut w = WeightVectorReduced::new_default();
        let before = w.weights;
        let u = test_u([0; MAX_P]);

        w.update(&img, &u, 5, 10, 1, 1, 0); // e = 2*5-10 = 0

        // delta = (shift_right(sgn*0, ro)+1)>>1 = (0+1)>>1 = 0 for every weight
        assert_eq!(w.weights, before);
    }

    #[test]
    fn positive_error_increases_weight_when_u_positive() {
        let img = test_image();
        let mut w = WeightVectorReduced::new_default();
        let before = w.weights[0];
        let mut u_vals = [0i64; MAX_P];
        u_vals[0] = 100;
        let u = test_u(u_vals);

        w.update(&img, &u, 10, 0, 1, 1, 0); // e = 2*10-0 = 20, positive

        assert!(w.weights[0] >= before);
    }

    #[test]
    fn negative_error_decreases_weight_when_u_positive() {
        let img = test_image();
        let mut w = WeightVectorReduced::new_default();
        let before = w.weights[0];
        let mut u_vals = [0i64; MAX_P];
        u_vals[0] = 100;
        let u = test_u(u_vals);

        w.update(&img, &u, 0, 20, 1, 1, 0); // e = 2*0-20 = -20, negative

        assert!(w.weights[0] <= before);
    }

    #[test]
    fn weights_never_exceed_clip_bounds() {
        let img = test_image();
        let mut w = WeightVectorReduced::new_default();
        let mut u_vals = [0i64; MAX_P];
        u_vals[0] = 1 << (D + 2);
        let u = test_u(u_vals);

        // repeatedly push toward the upper bound
        for _ in 0..1000 {
            w.update(&img, &u, S_MAX, 0, 1, 1, 0);
        }

        for weight in w.weights.iter() {
            assert!(*weight >= W_MIN as i64 && *weight <= W_MAX as i64);
        }
    }

}