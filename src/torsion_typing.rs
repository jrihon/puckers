use ndarray::Array1;

// Which torsion type is going to be calculated
#[derive(Debug)]
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
    pub fn new(start: f64, end: f64, num: usize) -> BackboneCoordinates {
        BackboneCoordinates {
            x: Array1::linspace(start, end, num),
            y: Array1::linspace(start, end, num),
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


pub struct SphericalCoordinates {
    pub x : Array1<f64>,
    pub y : Array1<f64>,
    pub z : Array1<f64>,
    pub rho : f64,
    pub theta : Array1<f64>,
    pub phi : Array1<f64>,
    pub amount : usize,
}


impl SphericalCoordinates {
    pub fn new(num: usize, m_theta : usize, rhoo: f64) -> SphericalCoordinates {
        SphericalCoordinates {
            x : Array1::<f64>::zeros(num),
            y : Array1::<f64>::zeros(num),
            z : Array1::<f64>::zeros(num),
            rho : rhoo,
            theta : Array1::<f64>::zeros(m_theta),
            phi : Array1::<f64>::zeros(num),
            amount : num,
        }
    }

    pub fn polar_to_cartesian(&mut self, i : usize, m : usize) {
        self.x[i] = self.rho * self.theta[m].sin() * self.phi[i].cos();
        self.y[i] = self.rho * self.theta[m].sin() * self.phi[i].sin();
        self.z[i] = self.rho * self.theta[m].cos();     
    }
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

pub trait Dihedrals {}

impl Dihedrals for Peptide {}
impl Dihedrals for Furanose {}
impl Dihedrals for Pyranose {}



/// Implement it on all structs that axes (dynamic dispatching)
pub trait Axis {}

impl Axis for BackboneCoordinates {}
impl Axis for FurCoords {}
impl Axis for SphericalCoordinates {}
