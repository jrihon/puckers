use std::f64::consts::PI;
use ndarray::Array1;

use crate::arguments::Flags;
use crate::sixring::equidistance_sphere::equidistance_sphere;

const TO_RAD: f64 = PI / 180.;
const TO_DEG: f64 = 180. / PI ;
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

//-------------
//
// Structs to hold torsional values
//
//-------------
//
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



//-------------
//
// Structs to hold axes for torsion angle calculations
//
//-------------
//
/// The axes to iterate over for peptide-like molecules : 
/// Its extent is : [0 , 2pi] (rad)
/// Its extent is : [0 , 360] (degrees)
/// public `x` field : Array1<f64>
/// public `y` field : Array1<f64>
pub struct PeptideAxes {
    pub x : Array1<f64>,
    pub y : Array1<f64>,

}

impl PeptideAxes {
    /// Initialise the struct with an array of zeroes
    pub fn new(num: usize) -> PeptideAxes {
        PeptideAxes {
            x: Array1::linspace(0., 360., num),
            y: Array1::linspace(0., 360., num),
        }
    }
}

/// The axes to iterate over for fivering molecules : 
/// Its extent is : [-60, 60]
/// public `zx` field : Array1<f64>
/// public `zy` field : Array1<f64>
pub struct FuranoseAxes {
    pub zx : Array1<f64>,
    pub zy : Array1<f64>,

}

impl FuranoseAxes {
    /// Initialise the struct with a near-empty array
    pub fn new(num: usize) -> FuranoseAxes {
        FuranoseAxes {
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
pub struct SphericalAxes {
    pub rho : f64,
    pub theta : Array1<f64>,
    pub phi : Array1<f64>,
    pub amount : usize,
}


impl SphericalAxes {
    pub fn new(amount: usize, m_theta : usize, rho: f64) -> SphericalAxes {
        SphericalAxes {
            rho, // shorthand initialisation
            theta : Array1::<f64>::zeros(m_theta),
            phi : Array1::<f64>::zeros(amount),
            amount // shorthand initialisation
        }
    }
}

/// We implement the print to output method signature on Dihedrals, 
/// which will be implemented on Peptide, Furanose and Pyranose
pub trait Dihedrals {

    /// A `#` pound symbol is added by on the first line, to act as a comment symbol 
    /// for when one wants to easily parse it through numpy, shell scripts or as an easy identifier.
    fn print_values(self, flags : Flags);
}



impl Dihedrals for Peptide {

    fn print_values(self, flags : Flags) {

        let mut axis = PeptideAxes::new(flags.num as usize);

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

        let axis = FuranoseAxes::new(flags.num as usize);
        let num_f64 : f64 = flags.num as f64;


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
    }
}

impl Dihedrals for Pyranose {
    fn print_values(self, flags : Flags) {

        let mut axis = equidistance_sphere(flags.num);
        // if the --rad flags was not prompted, since we already have the accessed as spherical coordinates
        if !flags.rad {
            axis.theta = axis.theta.iter().map(|theta| theta * TO_DEG).collect::<Array1<f64>>();
            axis.phi = axis.phi.iter().map(|phi| phi * TO_DEG).collect::<Array1<f64>>();
        };

        println!("{} {} {} {} {} {}", // header
                 "#   ALPHA1",
                 "    ALPHA2",
                 "    ALPHA3",
                 "       RHO",
                 "     THETA",
                 "       PHI",
                 );

        let mut it: usize = 0; // iterate over the theta array

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
