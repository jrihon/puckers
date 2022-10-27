#![allow(dead_code, unused_imports)] // temporarily remove all annoying warnings

// import module(sixring) specific modules
mod equidistance_globe;
use crate::sixring::equidistance_globe::{GlobeCoordinates, equidistance_globe};


use ndarray::prelude::*;
use crate::arguments::Flags;
use crate::Torsions;

pub fn sixring(flags : &Flags) -> Torsions {
//    println!("sixring module");
    let globe = equidistance_globe(flags.num);

    print_globe_cartesians(globe);


    let t = Torsions {
        array1 : Array1::linspace(1., 2., 5),
        array2 : Array1::linspace(1., 2., 5),
    };

    t

}



fn print_globe_cartesians(globe : GlobeCoordinates) {
    let _sizeof = globe.x.len();

    for i in 0.._sizeof { println!("{:?}", (globe.x[i], globe.y[i], globe.z[i])) }
}
