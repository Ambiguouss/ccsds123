use crate::hyperspectral_image::HyperspectralImage;
use crate::consts::{OMEGA,S_MAX,S_MID,S_MIN,MAX_P,R};
use crate::utils::mod_star;
//returns scaled predicted sample
pub fn scaled_predicted_value(image:&HyperspectralImage, sigma:i64, dhat:i64, x:usize, y:usize, z:usize) -> i64 {
    if x==0 && y==0 && (z==0 || MAX_P==0) {
        return 2*S_MID;
    }
    if x==0 && y==0 {
        return 2*image.get(x,y,z-1) as i64;
    }
    let r = (mod_star(dhat+(1i64<<OMEGA)*(sigma-4*S_MID),R) >> (OMEGA+1))+2*S_MID+1;
    r.clamp(2*S_MIN,2*S_MAX+1)
}

//todo tests