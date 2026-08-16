use crate::hyperspectral_image::HyperspectralImage;
use crate::central_difference_cache::CentralDifferenceCache;
use crate::consts::{MAX_P,FULL_LEN};

pub fn central_local_difference(image : &HyperspectralImage, sigma: i64, x: usize, y: usize, z: usize) -> i64 {
    return 4*image.get(x,y,z) as i64 - sigma;
}

#[derive(Debug, PartialEq)]
struct DirectionalDifferences{
    north: i64,
    west: i64,
    north_west: i64,
}

fn directional_differences(image : &HyperspectralImage, sigma: i64, x: usize, y: usize, z: usize) -> DirectionalDifferences {
    let north = if y > 0 {
        4*image.get(x,y-1,z) as i64 - sigma
    } else {
        0
    };
    let west = if y > 0 && x > 0 {
        4*image.get(x-1,y,z) as i64 - sigma
    } else {
        north
    };
    let north_west = if y > 0 && x > 0 {
        4*image.get(x-1,y-1,z) as i64 - sigma
    } else {
        north
    };
    DirectionalDifferences { north, west, north_west}
}

#[derive(Debug, PartialEq)]
pub struct CentralLocalDifferenceVectorFull{
    pub values: [i64; FULL_LEN],
}
#[derive(Debug, PartialEq)]
pub struct CentralLocalDifferenceVectorReduced {
    pub values: [i64; MAX_P],
}

impl CentralLocalDifferenceVectorReduced {
    pub fn get(&self,t:usize)->i64 {
        self.values[t]
    }
}

impl CentralLocalDifferenceVectorFull {
    pub fn get(&self,t:usize) ->i64 {
        self.values[t]
    }
}

pub fn compute_cldv_full(image : &HyperspectralImage, sigma: i64,cache: &CentralDifferenceCache<MAX_P>, x: usize, y: usize, z: usize) -> CentralLocalDifferenceVectorFull {
    let mut values = [0i64; FULL_LEN];
    let DirectionalDifferences {north,west,north_west} = directional_differences(image,sigma,x,y,z);
    values[0]=north;
    values[1]=west;
    values[2]=north_west; 
    for (i, v) in cache.recent().enumerate() {
        values[i+3] = v;
    }
    CentralLocalDifferenceVectorFull {
        values
    }
}

pub fn compute_cldv_reduced(cache: &CentralDifferenceCache<MAX_P>) -> CentralLocalDifferenceVectorReduced {
    let mut values = [0i64; MAX_P];
    for (i, v) in cache.recent().enumerate() {
        values[i] = v;
    }
    CentralLocalDifferenceVectorReduced {
        values,
    }
}
#[cfg(test)]
impl CentralLocalDifferenceVectorReduced {
    pub(crate) fn new_for_test(values: [i64; MAX_P]) -> Self {
        CentralLocalDifferenceVectorReduced { values }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::hyperspectral_image::SampleOrder;
    // 3x3 spatial, 1 band, BIL, values 1..=9:
    // y=0: 1 2 3
    // y=1: 4 5 6
    // y=2: 7 8 9
    fn test_image() -> HyperspectralImage {
        HyperspectralImage {
            data: vec![1,2,3,4,5,6,7,8,9],
            x_size: 3, y_size: 3, z_size: 1,
            order: SampleOrder::BIL,
        }
    }

    #[test]
    fn central_local_difference_basic() {
        let img = test_image();
        // x=1,y=1 -> value=5; sigma arbitrary, e.g. 10
        assert_eq!(central_local_difference(&img, 10, 1, 1, 0), 4 * 5 - 10);
    }

    #[test]
    fn directional_top_row_all_zero() {
        let img = test_image();
        let d = directional_differences(&img, 7, 1, 0, 0);
        assert_eq!(d, DirectionalDifferences { north: 0, west: 0, north_west: 0 });
    }

    #[test]
    fn directional_left_column_w_and_nw_match_north() {
        let img = test_image();
        let d = directional_differences(&img, 3, 0, 1, 0);
        assert_eq!(d.west, d.north);
        assert_eq!(d.north_west, d.north);
    }

    #[test]
    fn directional_interior_general_case() {
        let img = test_image();
        let sigma = 6;
        let d = directional_differences(&img, sigma, 1, 1, 0);
        assert_eq!(d, DirectionalDifferences {
            north: 4 * img.get(1, 0, 0) as i64 - sigma,       // 4*2-6=2
            west: 4 * img.get(0, 1, 0) as i64 - sigma,        // 4*4-6=10
            north_west: 4 * img.get(0, 0, 0) as i64 - sigma,  // 4*1-6=-2
        });
    }
#[test]
    fn full_places_directional_first_then_bands() {
        let img = test_image();
        let mut cache: CentralDifferenceCache<MAX_P> = CentralDifferenceCache::new();
        cache.push(100);
        cache.push(200);

        let sigma = 6;
        let cldv = compute_cldv_full(&img, sigma, &cache, 1, 1, 0);
        let d = directional_differences(&img, sigma, 1, 1, 0);

        assert_eq!(cldv.values[0], d.north);
        assert_eq!(cldv.values[1], d.west);
        assert_eq!(cldv.values[2], d.north_west);
        assert_eq!(cldv.values[3], 200); // most recent pushed first
        assert_eq!(cldv.values[4], 100);
        // remaining slots (if MAX_P > 2) should be untouched zeros
    }

    #[test]
    fn reduced_only_contains_bands_no_directional() {
        let mut cache: CentralDifferenceCache<MAX_P> = CentralDifferenceCache::new();
        cache.push(7);
        cache.push(8);

        let cldv = compute_cldv_reduced(&cache);

        assert_eq!(cldv.values[0], 8);
        assert_eq!(cldv.values[1], 7);
    }
}