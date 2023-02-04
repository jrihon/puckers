#![allow(dead_code, unused_variables)] // temporarily remove all annoying warnings

// import module(sixring) specific modules
mod equidistance_globe;
mod geometry;
mod local_elevation;
mod reconstruct_ring;
mod ring_partition;

use crate::sixring::equidistance_globe::equidistance_globe;

use crate::arguments::Flags;
use crate::Torsions;
use ndarray::prelude::*;

use self::ring_partition::RingPartition;

pub fn sixring(flags: Flags) -> Torsions {
    //    println!("sixring module");
    let globe = equidistance_globe(flags.num);

    let projection =
        local_elevation::cremerpople_evelation(&globe).projection_and_partition(flags.num);

    let vec_of_pyranoses = reconstruct_ring::reconstruct_coordinates(
        &projection,
        flags.num,
        local_elevation::cremerpople_evelation(&globe), // Zj matrix again
    );

    Torsions {
        array1: Array1::linspace(1., 2., 1),
        array2: Array1::linspace(1., 2., 1),
    }
}

//fn print_globe_cartesians(globe: GlobeCoordinates) {
//    for i in 0..globe.x.len() {
//        println!("{:?}", (globe.x[i], globe.y[i], globe.z[i]))
//    }
//}
//
//fn print_globe_sphericals(globe: GlobeCoordinates) {
//    let mut idx_theta: usize = 0;
//
//    for i in 0..globe.phi.len() {
//        // every new circle, we start off at phi == 0.0 rad
//        if (globe.phi[i] == 0.0) && i != 0 {
//            idx_theta += 1
//        };
//
//        println!("{:?}", (globe.rho, globe.theta[idx_theta], globe.phi[i]))
//    }
//}
