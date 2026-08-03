mod hyperspectral_image;
pub use hyperspectral_image::HyperspectralImage;
mod central_difference_cache;
mod local_difference_vector;
mod local_sum;
mod weight_vector;
mod consts;
mod predict;
mod utils;
pub fn clip(x: i32, bounds: (i32, i32)) -> i32 {
    let (x_min,x_max) = bounds;
    x.clamp(x_min, x_max)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn within_bounds() {
        assert_eq!(clip(5,(0,10)),5)
    }
    
    #[test]
    fn above_max() {
        assert_eq!(clip(15,(0,10)),10)
    }

    #[test]
    fn below_min() {
        assert_eq!(clip(-3,(4,10)),4)
    }
}