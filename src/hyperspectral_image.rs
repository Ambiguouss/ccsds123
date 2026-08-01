enum SampleOrder {
    BSQ,
    BIP,
    BIL,
}



pub struct HyperspectralImage {
    data: Vec<i16>,
    x_size: usize,
    y_size: usize,
    z_size: usize,   
    order: SampleOrder
}

impl HyperspectralImage {
    fn get(&self, x: usize, y: usize, z: usize) -> i16 {
        let idx = match self.order {
            SampleOrder::BSQ => z * self.y_size * self.x_size + y * self.x_size + x,
            SampleOrder::BIP => y * self.x_size * self.z_size + x * self.z_size + z,
            SampleOrder::BIL => y * self.z_size * self.x_size + z * self.x_size + x,
        };
        self.data[idx]
    }

    fn get_coords(&self, idx: usize) -> (usize,usize,usize) {
        if matches!(self.order,SampleOrder::BIL) {
            let x = idx % self.x_size;
            let z = idx/self.x_size % self.z_size;
            let y = idx/(self.x_size*self.z_size);
            return (x,y,z); 
        }
        panic!("unsuported order (only BIL supported for now)");
    }

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
    fn get_coords_roundtrip() {
        let img = test_image();
        // idx 4 -> x=1, y=1, z=0 in this layout (BIL, x_size=3, z_size=1)
        assert_eq!(img.get_coords(4), (1, 1, 0));
    }

}
