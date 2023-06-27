use std::f64::consts::PI;

use ndarray::Array1;

use crate::arguments::Flags;
use crate::torsion_typing::{Furanose, FuranoseAxes, Dihedrals};

const FOURPIOVERFIVE : f64 = (4. * PI) / 5.;

/// Generate the torsion angles to use as restraints for furanose molecules
///
/// The original formula is, derived from Altona-Sundaralingam and Cremer-Pople :
/// Zx = (nu1 + nu3) / ( 2cos(4pi/5))
/// Zy = (nu1 - nu3) / ( 2sin(4pi/5))
///
///
/// This has been rearranged to get : 
///
/// nu1 = ( (zx * 2cos(4pi/5)) + (zy * 2sin(4pi/5)) )/ 2
/// nu3 = ( (zx * 2cos(4pi/5)) - (zy * 2sin(4pi/5)) )/ 2
pub fn fivering(flags : &Flags) -> Furanose {
    
    // Derive torsion angles from the given axes
    let polars = FuranoseAxes::new(flags.num as usize);

    // Setup variable
    let amount : u64 = flags.num * flags.num;
    let num_f64 : f64 = flags.num as f64;

    // Initialise equation-specific constants
    let denominator_x : f64 = 2. * FOURPIOVERFIVE.cos();
    let denominator_y : f64 = 2. * FOURPIOVERFIVE.sin();

    // Instance Furanose struct
    let mut f = Furanose::new(amount as usize);

    let mut x : f64;
    let mut y : f64;

    for i in 0..amount as usize {
        // Calculate indexes for the array axises
        x = (i as f64 / num_f64).floor(); // X axis, returns with floor
        y = i as f64 % num_f64; // Y axis, return with modulo

        // fill out the array
        f.nu1[i] = (( polars.zx[x as usize] * denominator_x ) + ( polars.zy[y as usize] * denominator_y)) / 2.;
        f.nu3[i] = (( polars.zx[x as usize] * denominator_x ) - ( polars.zy[y as usize] * denominator_y)) / 2.;
    }

    // Make values ORCA-ready
    f.nu1 = f.nu1.iter().map(|x| if x < &0. { x + 360.} else {*x}).collect::<Array1<f64>>();
    f.nu3 = f.nu3.iter().map(|x| if x < &0. { x + 360.} else {*x}).collect::<Array1<f64>>();

    f
}
