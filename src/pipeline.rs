use crate::hyperspectral_image::HyperspectralImage;
use crate::weight_vector::WeightVectorReduced;
use crate::predict::scaled_predicted_value;
use crate::utils::dot_product;
use crate::local_difference_vector::{compute_cldv_reduced,central_local_difference};
use crate::consts::MAX_P;
use crate::central_difference_cache::CentralDifferenceCache;
use crate::local_sum::neighbor_oriented_sum;


/// Runs the CCSDS-123.0-B-2 adaptive predictor (reduced, neighbor-oriented mode)
/// over a whole image and returns the scaled predicted sample for every sample,
/// in BIP traversal order (band z innermost, then x, then y).
pub struct Predictor{
    image: HyperspectralImage
}
impl Predictor{
    pub fn new(image: HyperspectralImage) -> Self {
        Self { image }
    }

    pub fn run(&self) -> Vec<i64> {
        let mut weight_vectors: Vec<WeightVectorReduced> = (0..self.image.z_size)
        .map(|_| WeightVectorReduced::new_default())
        .collect();
        let mut caches: Vec<CentralDifferenceCache<MAX_P>> = (0..self.image.x_size * self.image.y_size)
        .map(|_| CentralDifferenceCache::new())
        .collect();
        let mut output = Vec::new();
        for y in 0..self.image.y_size{
            for x in 0..self.image.x_size{
                for z in 0..self.image.z_size{
                    let sigma = neighbor_oriented_sum(&self.image,x,y,z);
                    let u = compute_cldv_reduced(&caches[y+x*self.image.y_size]);
                    let d = central_local_difference(&self.image,sigma,x,y,z);
                    let s_pred = scaled_predicted_value(&self.image,sigma,dot_product(&weight_vectors[z].weights,&u.values),x,y,z);
                    output.push(s_pred);
                    caches[y+x*self.image.y_size].push(d);
                    weight_vectors[z].update(&self.image,&u,self.image.get(x,y,z) as i64,s_pred,x,y,z);
                }
            }
        }
        output
    }
}