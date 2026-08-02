use crate::hyperspectral_image::HyperspectralImage;
use crate::central_difference_cache::CentralDifferenceCache;

fn central_local_difference(image : &HyperspectralImage, sigma: i32, x: usize, y: usize, z: usize) -> i32 {
    return 4*image.get(x,y,z) as i32 - sigma;
}

#[derive(Debug, PartialEq)]
struct DirectionalDifferences{
    north: i32,
    west: i32,
    north_west: i32,
}

fn directional_differences(image : &HyperspectralImage, sigma: i32, x: usize, y: usize, z: usize) -> DirectionalDifferences {
    let north = if y > 0 {
        4*image.get(x,y-1,z) as i32 - sigma
    } else {
        0
    };
    let west = if y > 0 && x > 0 {
        4*image.get(x-1,y,z) as i32 - sigma
    } else {
        north
    };
    let north_west = if y > 0 && x > 0 {
        4*image.get(x-1,y-1,z) as i32 - sigma
    } else {
        north
    };
    DirectionalDifferences { north, west, north_west}
}

#[derive(Debug, PartialEq)]
enum CentralLocalDifferenceVector<const MAX_P:usize>{
    Full {
        directional: DirectionalDifferences,
        previous_bands: [i32; MAX_P],
    },
    Reduced {
        previous_bands: [i32; MAX_P],
    }
}

fn compute_cldv_full<const MAX_P: usize>(image : &HyperspectralImage, sigma: i32,cache: &CentralDifferenceCache<MAX_P>, x: usize, y: usize, z: usize) -> CentralLocalDifferenceVector<MAX_P> {
    let mut previous_bands = [0i32; MAX_P];
    for (i, v) in cache.recent().enumerate() {
        previous_bands[i] = v;
    }
    CentralLocalDifferenceVector::Full {
        directional : directional_differences(image,sigma,x,y,z),
        previous_bands,
    }
}

fn compute_cldv_reduced<const MAX_P: usize>(cache: &CentralDifferenceCache<MAX_P>) -> CentralLocalDifferenceVector<MAX_P> {
    let mut previous_bands = [0i32; MAX_P];
    for (i, v) in cache.recent().enumerate() {
        previous_bands[i] = v;
    }
    CentralLocalDifferenceVector::Reduced {
        previous_bands,
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
            north: 4 * img.get(1, 0, 0) as i32 - sigma,       // 4*2-6=2
            west: 4 * img.get(0, 1, 0) as i32 - sigma,        // 4*4-6=10
            north_west: 4 * img.get(0, 0, 0) as i32 - sigma,  // 4*1-6=-2
        });
    }

    #[test]
    fn compute_cldv_reduced_empty_cache() {
        let cache: CentralDifferenceCache<3> = CentralDifferenceCache::new();
        let cldv = compute_cldv_reduced(&cache);
        assert_eq!(cldv, CentralLocalDifferenceVector::Reduced { previous_bands: [0, 0, 0] });
    }

    #[test]
    fn compute_cldv_reduced_partial_cache() {
        let mut cache: CentralDifferenceCache<3> = CentralDifferenceCache::new();
        cache.push(5);
        cache.push(9);
        let cldv = compute_cldv_reduced(&cache);
        // only 2 pushed; 3rd slot stays default 0
        assert_eq!(cldv, CentralLocalDifferenceVector::Reduced { previous_bands: [9, 5, 0] });
    }

    #[test]
    fn compute_cldv_reduced_full_cache() {
        let mut cache: CentralDifferenceCache<3> = CentralDifferenceCache::new();
        cache.push(1);
        cache.push(2);
        cache.push(3);
        let cldv = compute_cldv_reduced(&cache);
        assert_eq!(cldv, CentralLocalDifferenceVector::Reduced { previous_bands: [3, 2, 1] });
    }

    #[test]
    fn compute_cldv_full_combines_directional_and_bands() {
        let img = test_image();
        let mut cache: CentralDifferenceCache<3> = CentralDifferenceCache::new();
        cache.push(11);
        cache.push(22);

        let sigma = 6;
        let cldv = compute_cldv_full(&img, sigma, &cache, 1, 1, 0);

        assert_eq!(cldv, CentralLocalDifferenceVector::Full {
            directional: directional_differences(&img, sigma, 1, 1, 0),
            previous_bands: [22, 11, 0],
        });
    }
}