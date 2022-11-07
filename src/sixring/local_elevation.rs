///  Derive the local elevation of all the puckering modes and every mode's atomic elevation
///
///  " General definition of ring puckering coordinates, Cremer, DT and Pople, JA "
///  Journal of the American Chemical Society. doi.org/10.1021/ja00839a011
///
/// 1.        [                 term 1                         ]   [          term 2              ]
///     z_j = sqrt(2/N) * q_m * cos[phi_m + (2pi * m * (j-1))/N] + 1/sqrt(6) * q_(m+1) * (-1)^(j-1)
///
/// 2. For N = 6 and m = 2
///     z_j = sqrt(1/3) * q_2 * cos[phi_2 + (2pi * (j-1))/3] + 1/sqrt(6) * q_3 * (-1)^(j-1)
///
/// 3. Convert the puckering parameters to spherical coordinates
///     q_2 = Q * sin(theta)
///     q_3 = Q * cos(theta)
///     phi_2 = phi
///             [                  term 1                         ]   [             term 2                ]
///     z_j = [ sqrt(1/3) * sin(theta) * cos[phi + (2pi * (j-1))/3] + 1/sqrt(6) * cos(theta) * (-1)^(j-1) ] * Q   (=> fn calculate_local_elevation(...))
///
///   3.1 Where, for i = 1..6 : 
///     constant1 = (2pi * (j-1))/3   --> Vec<f64, 6>
///     constant2 = (-1)^(j-1)        --> Vec<f64, 6>
///
///   3.2 and the following are constant too : 
///     sqrt_one_over_three = sqrt(1/3)
///     one_over_sqrt_six   = 1 / sqrt(6)
///
///

// imports
use ndarray::{Array,Array2, Ix2};
use crate::sixring::equidistance_globe::GlobeCoordinates;
use crate::sixring::equidistance_globe::TWOPI as TWOPI;

// CONSTANTS
const Z_SIZE: usize = 6;

pub fn cremerpople_evelation(globe : &GlobeCoordinates) -> Array2<f64> {

    // Instance several variables
    let globe_size: usize = globe.phi.len();

    // 6 atomic elevations (Z_j) for any set of (r, theta, phi)
    let mut z: Array<f64, Ix2> = Array2::zeros((globe_size, Z_SIZE));

    // Set two constant values
    let constant1: Vec<f64> = constant_from_term1();
    let constant2: Vec<f64> = constant_from_term2();

    // Set some more constant values
    let sqrt_one_over_three: f64 = (1_f64/3_f64).sqrt() ;
    let one_over_sqrt_six: f64 = 1_f64/6_f64.sqrt() ;

    let mut idx_theta: usize = 0;
    for i in 0..globe_size { 
        // the way we generate the globe is in layered circles.
        // every new circle, we start off again at phi == 0.0
        // if we move to a new layer; we have to the next theta value
        // NB :the theta and phi arrays are not of the same length
        if (globe.phi[i] == 0.0) && !(i == 0) { idx_theta += 1 };

        for j in 1..=6 {
            z[[i, j]] = calculate_local_elevation(globe.rho, globe.theta[idx_theta], globe.phi[i], &constant1, &constant2, j,
                                                  sqrt_one_over_three, one_over_sqrt_six)
        }
    }

    z // return local elevation of all the conformations

}


fn calculate_local_elevation(rho : f64, theta: f64, phi: f64, c1 : &Vec<f64>,  c2 : &Vec<f64>, j: usize,
                             onethree: f64, onesix: f64) -> f64 {
    
    let term1 = onethree * theta.sin() * (phi + c1[j]).cos(); // first term of the equation
    let term2 = onesix * theta.cos() * c2[j]; // second term of the equation

    (term1 + term2) * rho // multiply by rho, which was pushed out by both terms to the outside of the equation
}


fn constant_from_term1() -> Vec<f64> {

    vec![1.,2.,3.,4.,5.,6.].iter()
                           .map(|j| TWOPI * ((j - 1.) / 3.))
                           .collect::<Vec<f64>>()
}


fn constant_from_term2() -> Vec<f64> {

    vec![1.,2.,3.,4.,5.,6.].into_iter()
                           .map(|j| -1_f64.powf(j - 1.))
                           .collect::<Vec<f64>>()
}

#[test]
fn test_iterating_over_array1() {

    // Rust method
    let vec1: Vec<f64> = vec![1.,2.,3.,4.,5.,6.].iter()
                           .map(|j| TWOPI * ((j - 1.) / 3.))
                           .collect::<Vec<f64>>();

    // Manual method
    let mut vec2: Vec<f64> = Vec::with_capacity(Z_SIZE);

    for j in 1..=6  {
        vec2.push((TWOPI * (j as f64 - 1.)) / 3.)
    };
    
    assert_eq!(vec1, vec2)
}

#[test]
fn test_iterating_over_array2() {

    // Rust method

    let vec1 = vec![1.,2.,3.,4.,5.,6.].into_iter()
                           .map(|j| -1_f64.powf(j - 1.))
                           .collect::<Vec<f64>>();
                           
    let mut vec2: Vec<f64> = Vec::with_capacity(Z_SIZE);
    for j in 1..=6  {
        vec2.push(-1_f64.powf(j as f64 - 1.))
    }


    assert_eq!(vec1, vec2)
}
