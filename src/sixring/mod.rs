#![allow(dead_code, unused_variables)] // temporarily remove all annoying warnings

// import module(sixring) specific modules
mod equidistance_globe;
mod local_elevation;
mod ring_partition;
mod reconstruct_ring;

use crate::sixring::equidistance_globe::{GlobeCoordinates, equidistance_globe};


use ndarray::prelude::*;
use crate::arguments::Flags;
use crate::Torsions;

use self::ring_partition::RingPartition;

pub fn sixring(flags : Flags) -> Torsions {
//    println!("sixring module");
    let globe = equidistance_globe(flags.num);

    let ringpartition = local_elevation::cremerpople_evelation(&globe).projection_and_partition(flags.num);

//    println!("{:?}", ringpartition.qp);
//    print_globe_cartesians(globe);
//    print_globe_sphericals(globe);


    // We will create a Vec<MolReconstruction> with size : flags.num
    // This makes writing the code far more manageable
    // let x : Vec<MolReconstruction> = Vec::with_capacitiy(flags.num)
    // for i in 0..flags.size {
    //     x[i] = reconstruct_molecule(ringpartition)
    // };
    //

    Torsions {
        array1 : Array1::linspace(1., 2., 1),
        array2 : Array1::linspace(1., 2., 1),
    }

}



fn print_globe_cartesians(globe : GlobeCoordinates) {

    for i in 0..globe.x.len() { println!("{:?}", (globe.x[i], globe.y[i], globe.z[i])) }
}


fn print_globe_sphericals(globe : GlobeCoordinates) {

    let mut idx_theta: usize = 0;

    for i in 0..globe.phi.len() { // every new circle, we start off at phi == 0.0 rad
        if (globe.phi[i] == 0.0) && i != 0 {
            idx_theta += 1 
        };

        println!("{:?}", (globe.rho, globe.theta[idx_theta], globe.phi[i])) 
    }
}
