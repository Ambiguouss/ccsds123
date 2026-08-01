use crate::hyperspectral_image::HyperspectralImage;

type LocalSumFn = fn(&HyperspectralImage,usize,usize,usize) -> i32;

fn neighbor_oriented_sum(image: &HyperspectralImage, x: usize, y: usize, z: usize) -> i32 {
    if x==0 && y==0 {
        return 0;
    }
    if x==0 {
        return 2*(self.get(x,y-1,z) as i32 +self.get(x+1,y-1,z) as i32);
    }
    if y==0 {
        return 4*self.get(x-1,y,z) as i32;
    }
    if x==self.x_size-1 {
        return self.get(x-1,y,z) as i32
            +self.get(x-1,y-1,z) as i32
                + 2*self.get(x,y-1,z) as i32;
    }
    return self.get(x-1,y,z) as i32
        +self.get(x-1,y-1,z) as i32
            +self.get(x,y-1,z) as i32
                +self.get(x+1,y-1,z) as i32;
}

pub fn sigma(local_sum: LocalSumFn, image: &HyperspectralImage, x: usize, y: usize, z: usize) -> i32 {
    local_sum(image,x,y,z)
}

#[cfg(test)]
mod tests {
    use super::*;

    // 3x3 spatial, 1 band, BIL, values 1..=9 laid out row-major:
    // y=0: 1 2 3
    // y=1: 4 5 6
    // y=2: 7 8 9
    fn test_image() -> HyperspectralImage {
        HyperspectralImage {
            data: vec![1,2,3,4,5,6,7,8,9],
            x_size: 3,
            y_size: 3,
            z_size: 1,
            order: SampleOrder::BIL,
        }
    }

    #[test]
    fn corner_x0_y0_is_zero() {
        let img = test_image();
        assert_eq!(neighbor_oriented_sum(img,0, 0, 0), 0);
    }

    #[test]
    fn left_edge_x0_y_nonzero() {
        // x=0, y=1 -> 2*(get(0,0,0) + get(1,0,0)) = 2*(1+2) = 6
        let img = test_image();
        assert_eq!(neighbor_oriented_sum(img,0, 1, 0), 6);
    }

    #[test]
    fn top_edge_y0_x_nonzero() {
        // y=0, x=1 -> 4*get(0,0,0) = 4*1 = 4
        let img = test_image();
        assert_eq!(neighbor_oriented_sum(img,1, 0, 0), 4);
    }

    #[test]
    fn right_edge_x_max() {
        // x=2 (x_size-1), y=1
        // get(1,1,0) + get(1,0,0) + 2*get(2,0,0) = 5 + 2 + 2*3 = 13
        let img = test_image();
        assert_eq!(neighbor_oriented_sum(img,2, 1, 0), 13);
    }

    #[test]
    fn interior_general_case() {
        // x=1, y=1
        // get(0,1,0) + get(0,0,0) + get(1,0,0) + get(2,0,0) = 4+1+2+3 = 10
        let img = test_image();
        assert_eq!(neighbor_oriented_sum(img,1, 1, 0), 10);
    }
}