use ndarray::Array1;

use crate::arguments::Flags;
use crate::sixring::equidistance_sphere::equidistance_sphere;

// Which torsion type is going to be calculated
//#[derive(Debug, Clone)]
pub enum TorsionType {
    Backbone,
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
pub struct Peptide {
    pub phi : Array1<f64>,
    pub psi : Array1<f64>,
}

impl Peptide {
    pub fn new(_sizeof : usize) -> Peptide {
        Peptide {
            phi : Array1::zeros(_sizeof),
            psi : Array1::zeros(_sizeof),
        }
    }
}



/// the `nu` dihedrals, according to the IUPAC nomenclature convention
pub struct Furanose {
    pub nu1 : Array1<f64>,
    pub nu3 : Array1<f64>,
}

impl Furanose {
    pub fn new(_sizeof : usize) -> Furanose {
        Furanose {
            nu1 : Array1::zeros(_sizeof),
            nu3 : Array1::zeros(_sizeof),
        }
    }
}




/// the `alpha` dihedrals according to the Strauss-Piccket (SP) pyranose puckering formalism
pub struct Pyranose {
    pub alpha1 : Array1<f64>,
    pub alpha2 : Array1<f64>,
    pub alpha3 : Array1<f64>,
}

impl Pyranose {
    pub fn new(sphere_size : usize) -> Pyranose {
         Pyranose {
            alpha1 : Array1::zeros(sphere_size),
            alpha2 : Array1::zeros(sphere_size),
            alpha3 : Array1::zeros(sphere_size),
        }       
    }
}

pub struct BackboneCoordinates {
    pub x : Array1<f64>,
    pub y : Array1<f64>,

}

impl BackboneCoordinates {
    pub fn new(num: usize) -> BackboneCoordinates {
        BackboneCoordinates {
            x: Array1::linspace(0., 360., num),
            y: Array1::linspace(0., 360., num),
        }
    }
}

pub struct FurCoords {
    pub zx : Array1<f64>,
    pub zy : Array1<f64>,

}

impl FurCoords {
    pub fn new(num: usize) -> FurCoords {
        FurCoords {
            zx: Array1::linspace(-60., 60., num),
            zy: Array1::linspace(-60., 60., num),
        }
        
    }
    
}


#[derive(Debug)]
pub struct SphericalCoordinates {
//    pub x : Array1<f64>,
//    pub y : Array1<f64>,
//    pub z : Array1<f64>,
    pub rho : f64,
    pub theta : Array1<f64>,
    pub phi : Array1<f64>,
    pub amount : usize,
}


impl SphericalCoordinates {
    pub fn new(num: usize, m_theta : usize, rhoo: f64) -> SphericalCoordinates {
        SphericalCoordinates {
//            x : Array1::<f64>::zeros(num),
//            y : Array1::<f64>::zeros(num),
//            z : Array1::<f64>::zeros(num),
            rho : rhoo,
            theta : Array1::<f64>::zeros(m_theta),
            phi : Array1::<f64>::zeros(num),
            amount : num,
        }
    }

//    pub fn polar_to_cartesian(&mut self, i : usize, m : usize) {
//        self.x[i] = self.rho * self.theta[m].sin() * self.phi[i].cos();
//        self.y[i] = self.rho * self.theta[m].sin() * self.phi[i].sin();
//        self.z[i] = self.rho * self.theta[m].cos();     
//    }
}



/// Implement it on all structs that hold torsion types (dynamic dispatching)

/// NOTE: 
/// Ok, this is what we do !
/// We implement the print to output method signature on Dihedrals, 
/// which will be implemented on Peptide, Furanose and Pyranose
/// Then, we can pass in the axis and the flags as arguments to the method
/// like so :
/// impl Dihedrals for Pyranose {
///     pub fn print_to_stdout(self, axes: impl Axes, flags: Flags) -> () {
///
///         if flags.twopi {}
///         if flags.rad {}
///         if flags.axis {}
///
///         for i in iterate {
///             println!("{}", self.rho, self.theta, self.phi)
///         }
///
///     }
/// }

pub trait Dihedrals {
    fn print_to_stdout(&self, flag : Flags);
    // the problem with this program is that we want to return the axis alongside the torsion
    // angles
    // since we had to return the axes as trait object, we cannot access its field
    // as they are unknown and therefor the compiler cannot guarantee it is there.
    // Technically, this could work with an unsafe block, but I feel that is unreasonable.
    //
    // The alternative is to recalculate the entire axis, which is what I'll do.
    // Not the best, but definitely not the worst idea. It is probably a couple extra milliseconds
    // extra time to calculate, but negligible
    // In all fairness, it is probably better this way, as here we can allow mutation of the
    // axes, whereas with a trait object I am not all too certain.
    //
    // https://docs.rs/float-pretty-print/latest/src/float_pretty_print/lib.rs.html#48
    // This is a lib to pretty print stuff.
    // I'll mention I took it from here, but I just want to do it manually myself
    //
    // A pound sign is added at the start of the column-header line (line 0), to make it 
    // easily parseable from numpy or shell

}

impl Dihedrals for Peptide {

    fn print_to_stdout(&self, flag : Flags) {

        let axis = BackboneCoordinates::new(flag.num as usize);

        let _sizeof: usize = flag.num as usize * flag.num as usize;
        let num_f64 = flag.num as f64;
        let mut x : f64;
        let mut y : f64;

        println!("{} {} {} {}", // header
                 "#      PHI",
                 "       PSI",
                 "         X",
                 "         Y"
                 );

        for i in 0.._sizeof {

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

    fn print_to_stdout(&self, flag : Flags) {
        let _sizeof: usize = flag.num as usize * flag.num as usize;

        let axis = FurCoords::new(flag.num as usize);
        let mut x : f64;
        let mut y : f64;
        let num_f64 : f64 = flag.num as f64;

        println!("{} {} {} {}", // header
                 "#      NU1",
                 "       NU3",
                 "        Zx",
                 "        Zy"
                 );

        for i in 0.._sizeof {

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
    fn print_to_stdout(&self, flag : Flags) {

        let axis = equidistance_sphere(flag.num);

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



/// Implement it on all structs that axes (dynamic dispatching)
pub trait Axis {}

impl Axis for BackboneCoordinates {}
impl Axis for FurCoords {}
impl Axis for SphericalCoordinates {}
