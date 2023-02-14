use std::f64::consts::PI;

use crate::arguments::Flags;
use crate::torsion_typing::{Furanose, FurCoords, Dihedrals};




pub fn fivering(flags : &Flags) -> Box<dyn Dihedrals> {
    
    // Derive torsion angles from the given axes
    let polars = FurCoords::new(flags.num as usize);

    // Setup variable
    let _sizeof : u64 = flags.num * flags.num;
    let num_f64 : f64 = flags.num as f64;

    // Initialise equation-specific constants
    const FOURPIOVERFIVE : f64 = (4. * PI) / 5.;
    let denominator_x : f64 = 2. * FOURPIOVERFIVE.cos();
    let denominator_y : f64 = 2. * FOURPIOVERFIVE.sin();

    // Instance Furanose struct
    let mut f = Furanose::new(_sizeof as usize);

    let mut x : f64;
    let mut y : f64;

    for i in 0.._sizeof {
        // Calculate indexes for the array axises
        x = (i as f64 / num_f64).floor(); // X axis, returns with floor
        y = i as f64 % num_f64; // Y axis, return with modulo

        f.nu1[i as usize] = (( polars.zx[x as usize] * denominator_x ) + ( polars.zy[y as usize] * denominator_y)) / 2.;
        f.nu3[i as usize] = (( polars.zx[x as usize] * denominator_x ) - ( polars.zy[y as usize] * denominator_y)) / 2.;
    }

    Box::new(f)
}
