use crate::consts::{MAX_P,FULL_LEN,OMEGA};

pub struct WeightVectorFull{
    pub weights : [i32;FULL_LEN],
}


pub struct WeightVectorReduced{
    pub weights : [i32;MAX_P],
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
}

impl WeightVectorFull{
    pub fn new_default() -> Self{
        let mut weights = [0i32; FULL_LEN];
        let bands: [i32; MAX_P] = WeightVectorReduced::new_default().weights;
        for (i, v) in bands.iter().enumerate() {
            weights[i + 3] = *v;
        }
        WeightVectorFull { weights }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

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
}