use ndarray::Array1;
use std::f64::consts::PI;

use crate::arguments::Flags;
use crate::torsion_typing::Furanose;




pub fn fivering(flags : Flags) -> Furanose {
    
    Furanose::new(flags.num) 
}



impl Furanose {
    
    fn new(num : u64) -> Furanose {
        // Derive torsion angles from the given axes
        
        let ax1 : Array1<f64> = Array1::linspace(-60., 60., num as usize);
        let ax2 = ax1.clone();
        // Setup variable
        let _sizeof : u64 = num * num;
        let num_f64 : f64 = num as f64;

        // Initialise equation-specific constants
        const FOURPIOVERFIVE : f64 = (4. * PI) / 5.;
        let denominator_x : f64 = 2. * FOURPIOVERFIVE.cos();
        let denominator_y : f64 = 2. * FOURPIOVERFIVE.sin();

        // Initialise Torsions struct
        let mut f = Furanose {
            nu1 : Array1::zeros(_sizeof as usize),
            nu3 : Array1::zeros(_sizeof as usize),
        };

        let mut x : f64;
        let mut y : f64;

        for i in 0.._sizeof {
            // Calculate indexes for the array axises
            x = (i as f64 / num_f64).floor(); // X axis, returns with floor
            y = i as f64 % num_f64; // Y axis, return with modulo

            f.nu1[i as usize] = (( ax1[x as usize] * denominator_x ) + ( ax2[y as usize] * denominator_y)) / 2.;
            f.nu3[i as usize] = (( ax1[x as usize] * denominator_x ) - ( ax2[y as usize] * denominator_y)) / 2.;
        }

        f
    }

}
