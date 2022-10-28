#![allow(unused_mut)]
///
/// Derive the local elevation of all the puckering modes and every mode's atomic elevation
///
///
///
///
///
///
///
///
///

// imports
use ndarray::{Array,Array2, Ix2};
use crate::sixring::equidistance_globe::GlobeCoordinates;
use std::f64::consts::PI;

// CONSTANTS
const Z_SIZE: usize = 6;
const TWOPI: f64 = 2. * PI;

pub fn cremerpople_evelation(globe : &GlobeCoordinates) -> Array2<f64> {

    // Instance several variables
    let globe_size: usize = globe.phi.len();

    // 6 atomic elevations (Z_j) for any set of (r, theta, phi)
    let mut z: Array<f64, Ix2> = Array2::zeros((globe_size, Z_SIZE));

    // Set two constant values
    let sqrt_one_over_three: f64 = (1_f64/3_f64).sqrt() ;
    let one_over_sqrt_six: f64 = 1_f64/6_f64.sqrt() ;

    let constant1: Vec<f64> = constant_from_term1();
    let constant2: Vec<f64> = constant_from_term2();

    let mut idx_theta: usize = 0;
    for i in 0..globe_size { 
        // the way we generate the globe is in layered circles.
        // every new circle, we start off again at phi == 0.0
        // if we move to a new layer; we have to the next theta value
        // NB :the theta and phi arrays are not of the same length
        if (globe.phi[i] == 0.0) && !(i == 0) { idx_theta = idx_theta + 1 };

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
    let mut constant1: Vec<f64> = Vec::with_capacity(Z_SIZE);

    for j in 1..=6  {
        constant1[j - 1] = (TWOPI * (j as f64 - 1.)) / 3.
    }
    
    constant1 // return constant value in term 1
}


fn constant_from_term2() -> Vec<f64> {
    let mut constant2: Vec<f64> = Vec::with_capacity(Z_SIZE);
    
    for j in 1..=6  {
        constant2[j - 1] = -1_f64.powf(j as f64 - 1.)
    }

    constant2 // return constant value in term 2
}

