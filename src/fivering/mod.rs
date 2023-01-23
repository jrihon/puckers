use ndarray::Array1;
use std::f64::consts::PI;

use crate::arguments::Flags;
use crate::Torsions;




pub fn fivering(flags : Flags) -> Torsions {
    
    let axis_x : Array1<f64> = Array1::linspace(-60., 60., flags.num as usize);
    let axis_y = axis_x.clone();

    generate_nu1_and_nu3(flags.num, axis_x, axis_y) // returned value
}


fn generate_nu1_and_nu3(num : u32, axis_x: Array1<f64>, axis_y: Array1<f64>) -> Torsions {
    // Derive torsion angles from the given axes
    
    // Setup variable
    let _sizeof : u32 = num * num;
    let num_f64 : f64 = num as f64;

    // Initialise equation-specific constants
    const FOURPIOVERFIVE : f64 = (4. * PI) / 5.;
    let denominator_x : f64 = 2. * FOURPIOVERFIVE.cos();
    let denominator_y : f64 = 2. * FOURPIOVERFIVE.sin();

    // Initialise Torsions struct
    let mut t = Torsions::new(_sizeof as usize);

    let mut x : f64;
    let mut y : f64;

    for i in 0.._sizeof {
        // Calculate indexes for the array axises
        x = (i as f64 / num_f64).floor(); // X axis, returns with floor
        y = i as f64 % num_f64; // Y axis, return with modulo

        t.array1[i as usize] = (( axis_x[x as usize] * denominator_x ) + ( axis_y[y as usize] * denominator_y)) / 2.;
        t.array2[i as usize] = (( axis_x[x as usize] * denominator_x ) - ( axis_y[y as usize] * denominator_y)) / 2.;
    }

    t // return Torsions struct
}

