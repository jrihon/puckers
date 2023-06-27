#![allow(dead_code)]
use std::f64::consts::PI;
use ndarray::Array1;

use crate::arguments::Flags;
use crate::sixring::equidistance_sphere::equidistance_sphere;

const TO_RAD: f64 = PI / 180.;
const TO_DEG: f64 = 180. / PI ;
const TWOPI: f64 = 2. * PI;
// Which torsion type is going to be calculated
//#[derive(Debug, Clone)]
#[derive(Debug)]
pub enum TorsionType {
    Peptide,
    Fivering,
    Sixring,
}

///
/// Create structs to hold the torsion types in
///
/// The Peptide struct holds the data to iterate over the phi-psi backbone angles
///
/// The Furanose struct holds the data to iterate over the endocyclic dihedrals nu_1-nu_3
///
/// The Pyranose struct holds the data to iterate over the
/// Strauss-Piccket alpha1-alpha2-alpha3 improper dihedrals
///
///
///

/// the `phi-psi` dihedrals, which are the peptide backbone dihedrals in proteins
/// public `phi` field : Array1<f64>
/// public `psi` field : Array1<f64>
pub struct Peptide {
    pub phi : Array1<f64>,
    pub psi : Array1<f64>,
}

impl Peptide {
    /// Initialise the struct with an array of zeroes
    pub fn new(amount : usize) -> Peptide {
        Peptide {
            phi : Array1::zeros(amount),
            psi : Array1::zeros(amount),
        }
    }
}



/// the `nu` dihedrals, according to the IUPAC nomenclature convention
/// public `nu1` field : Array1<f64>
/// public `nu3` field : Array1<f64>
pub struct Furanose {
    pub nu1 : Array1<f64>,
    pub nu3 : Array1<f64>,
}

impl Furanose {
    /// Initialise the struct with an array of zeroes
    pub fn new(amount : usize) -> Furanose {
        Furanose {
            nu1 : Array1::zeros(amount),
            nu3 : Array1::zeros(amount),
        }
    }
}




/// the `alpha` dihedrals according to the Strauss-Piccket (SP) pyranose puckering formalism
/// public `alpha1` field : Array1<f64>
/// public `alpha2` field : Array1<f64>
/// public `alpha3` field : Array1<f64>
pub struct Pyranose {
    pub alpha1 : Array1<f64>,
    pub alpha2 : Array1<f64>,
    pub alpha3 : Array1<f64>,
}

impl Pyranose {
    /// Initialise the struct with an array of zeroes
    pub fn new(sphere_size : usize) -> Pyranose {
         Pyranose {
            alpha1 : Array1::zeros(sphere_size),
            alpha2 : Array1::zeros(sphere_size),
            alpha3 : Array1::zeros(sphere_size),
        }       
    }
}

                      
                             
                              
                              



/// The axes to iterate over for peptide-like molecules : 
/// Its extent is : [0 , 2pi] (rad)
/// Its extent is : [0 , 360] (degrees)
/// public `x` field : Array1<f64>
/// public `y` field : Array1<f64>
pub struct BackboneCoordinates {
    pub x : Array1<f64>,
    pub y : Array1<f64>,

}

impl BackboneCoordinates {
    /// Initialise the struct with an array of zeroes
    pub fn new(num: usize) -> BackboneCoordinates {
        BackboneCoordinates {
            x: Array1::linspace(0., 360., num),
            y: Array1::linspace(0., 360., num),
        }
    }
}

/// The axes to iterate over for fivering molecules : 
/// Its extent is : [-60, 60]
/// public `zx` field : Array1<f64>
/// public `zy` field : Array1<f64>
pub struct FurCoords {
    pub zx : Array1<f64>,
    pub zy : Array1<f64>,

}

impl FurCoords {
    /// Initialise the struct with a near-empty array
    pub fn new(num: usize) -> FurCoords {
        FurCoords {
            zx: Array1::linspace(-60., 60., num),
            zy: Array1::linspace(-60., 60., num),
        }
        
    }
    
}


/// The axes to iterate over for sixring molecules : 
/// public `rho` field : f64 . Standard value of 0.67
/// public `theta` field : Array1<f64>. [0, pi] or [0, 180]
/// public `phi` field : Array1<f64>. [0, 2pi] or [0, 360]
/// public `amount` field : Array1<f64>. The corrected amount of points to sample
pub struct SphericalCoordinates {
    pub rho : f64,
    pub theta : Array1<f64>,
    pub phi : Array1<f64>,
    pub amount : usize,
}


impl SphericalCoordinates {
    pub fn new(amount: usize, m_theta : usize, rho: f64) -> SphericalCoordinates {
        SphericalCoordinates {
            rho, // shorthand initialisation
            theta : Array1::<f64>::zeros(m_theta),
            phi : Array1::<f64>::zeros(amount),
            amount // shorthand initialisation
        }
    }
}

//pub fn print_to_stdout(puckers: impl Dihedrals, flags: Flags) {
//
//    match flags.torsion_type.unwrap() {
//        TorsionType::Peptide => (),
//        TorsionType::Fivering => (),
//        TorsionType::Sixring => (),
//    }
//}

/// We implement the print to output method signature on Dihedrals, 
/// which will be implemented on Peptide, Furanose and Pyranose
///
///
/// Design a trait to allow for dynamic dispatching of the torsion angles.
/// With this, we can then call the `self.print_to_stdout()` method on any of the 
/// torsion angle structs (Peptide, Furanose, Pyranose) and use it.
///
/// This eases our work and while we call one function, it behaves differently for every struct 
/// (which is essentially the concept of these trait objects)
///
pub trait Dihedrals {

    /// We pretty print the output with the use of the following library :
    /// https://docs.rs/float-pretty-print/latest/src/float_pretty_print/lib.rs.html#48
    /// I did not want to `use` it, but went through the src code to get exactly what I wanted.
    ///
    /// Additionally, a `#` pound symbol is added by on the first line, to act as a comment symbol 
    /// for when one wants to easily parse it through numpy, shell scripts or as an easy
    /// identifier.
    fn print_values(self, flags : Flags);

}



impl Dihedrals for Peptide {

    fn print_values(self, flags : Flags) {

        let mut axis = BackboneCoordinates::new(flags.num as usize);


        // if we want to print to radians instead of degrees
        if flags.rad {
            axis.x = axis.x.iter().map(|x| x * TO_RAD).collect::<Array1<f64>>();
            axis.y = axis.y.iter().map(|y| y * TO_RAD).collect::<Array1<f64>>();
        };


        let amount: usize = flags.num as usize * flags.num as usize;
        let num_f64 = flags.num as f64;
        let mut x : f64;
        let mut y : f64;

        println!("{} {} {} {}", // header
                 "#      PHI",
                 "       PSI",
                 "         X",
                 "         Y"
                 );

        for i in 0..amount {

            x = (i as f64 / num_f64).floor(); 
            y = i as f64 % num_f64; 

            println!("{:width$.precision$} {:width$.precision$} {:width$.precision$} {:width$.precision$}",
                     self.phi[i],
                     self.psi[i],
                     axis.x[x as usize],
                     axis.y[y as usize],
                     width=10, precision=3)
            }
    }

}
impl Dihedrals for Furanose {

    fn print_values(self, flags : Flags) {
        let amount: usize = flags.num as usize * flags.num as usize;

        let mut axis = FurCoords::new(flags.num as usize);
        let num_f64 : f64 = flags.num as f64;


        // output the values as the conformation's polar coordinate, Altona Sundaralingham
//        if flags.rad {
//
//            let mut x : f64;
//            let mut y : f64;
//            let mut zx : f64;
//            let mut zy : f64;
//            let mut ampl = Array1::<f64>::zeros(amount);
//            let mut phase = Array1::<f64>::zeros(amount);
//            for i in 0..amount {
//                x = (i as f64 / num_f64).floor();
//                y = i as f64 % num_f64; 
//
//                if axis.zx[x as usize] < 0. {
//                    zx = (&axis.zx[x as usize] + 360.) * TO_RAD
//                } else {
//                    zx = &axis.zx[x as usize] * TO_RAD
//                };
//                if axis.zy[y as usize] < 0. {
//                    zy = (&axis.zy[y as usize] + 360.) * TO_RAD
//                } else {
//                    zy = &axis.zy[y as usize] * TO_RAD
//                };
//
//                ampl[i] = (zx.powi(2) + zy.powi(2)).sqrt(); // amplitude rho
//                phase[i] = (zy).atan2(zx);
//            }
//            axis.zx = ampl;
//            axis.zy = phase;
//            println!("{} {} {} {}", // header
//                     "#      NU1",
//                     "       NU3",
//                     " AMPLITUDE",
//                     "     PHASE"
//                     );
//            
//            for i in 0..amount {
//
//                println!("{:width$.precision$} {:width$.precision$} {:width$.precision$} {:width$.precision$}",
//                         self.nu1[i],
//                         self.nu3[i],
//                         axis.zx[i],
//                         axis.zy[i],
//                         width=10, precision=3)
//            }
//
//        } else {
            println!("{} {} {} {}", // header
                     "#      NU1",
                     "       NU3",
                     "        Zx",
                     "        Zy"
                     );

            let mut x : f64;
            let mut y : f64;

            for i in 0..amount {

                x = (i as f64 / num_f64).floor();
                y = i as f64 % num_f64; 

                println!("{:width$.precision$} {:width$.precision$} {:width$.precision$} {:width$.precision$}",
                         self.nu1[i],
                         self.nu3[i],
                         axis.zx[x as usize],
                         axis.zy[y as usize],
                         width=10, precision=3)
            }

//        }
    }
}

impl Dihedrals for Pyranose {
    fn print_values(self, flags : Flags) {

        let mut axis = equidistance_sphere(flags.num);

        // if the --rad flags was not prompted, since we already have the accessed as spherical
        // coordinates
        if !flags.rad {
            axis.theta = axis.theta.iter().map(|theta| theta * TO_DEG).collect::<Array1<f64>>();
            axis.phi = axis.phi.iter().map(|phi| phi * TO_DEG).collect::<Array1<f64>>();
        };

        let mut it: usize = 0;

        println!("{} {} {} {} {} {}", // header
                 "#   ALPHA1",
                 "    ALPHA2",
                 "    ALPHA3",
                 "       RHO",
                 "     THETA",
                 "       PHI",
                 );
        for i in 0..axis.amount {
            if (axis.phi[i] == 0.0) && i != 0 { it += 1 }

            println!("{:width$.precision$} {:width$.precision$} {:width$.precision$} {:width$.precision$} {:width$.precision$} {:width$.precision$}",
                     self.alpha1[i],
                     self.alpha2[i],
                     self.alpha3[i], 
                     axis.rho,
                     axis.theta[it],
                     axis.phi[i],
                     width=10, precision=3)
        }
    }
}
