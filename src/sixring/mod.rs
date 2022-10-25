#![allow(dead_code, unused_imports)] // temporarily remove all annoying warnings

// import module(sixring) specific modules
mod equidistance_globe;


use ndarray::prelude::*;
use crate::arguments::Flags;
use crate::Torsions;

pub fn sixring() -> Torsions {
    println!("sixring module");
    equidistance_globe::equidistance_globe();

    let t = Torsions {
        array1 : Array1::linspace(1., 2., 5),
        array2 : Array1::linspace(1., 2., 5),
    };

    t

}
