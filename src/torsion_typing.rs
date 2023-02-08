use ndarray::Array1;

// Which torsion type is going to be calculated
#[derive(Debug)]
pub enum TorsionType {
    Backbone,
    Fivering,
    Sixring,
}

/// the `phi-psi` dihedrals, which are the peptide backbone dihedrals in proteins
pub struct Peptide {
    pub phi : Array1<f64>,
    pub psi : Array1<f64>,
}

/// the `nu` dihedrals, according to the IUPAC nomenclature convention
pub struct Furanose {
    pub nu1 : Array1<f64>,
    pub nu3 : Array1<f64>,
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

// This would hold either the struct Peptide, Furanose or Pyranose
//pub enum Dihedrals {
//    Peptide,
//    Furanose,
//    Pyranose
//}
// This would hold either the axes for the struct Peptide, Furanose or Pyranose
//pub enum Axis {
//    BackboneCoordinates,
//    PolarCoordinates,
//    SphericalCoordinates
//}
pub trait Dihedrals {}
pub trait Axis {}


// Holds both the Dihedrals and the Axes pertaining to the conformational sampling
//pub struct ConformationalSampling<'a> {
//    pub conformers : &'a Dihedrals,
//    pub axis : &'a Axis
//}


// This trait will be implemented on all three structs
//pub trait Output{
//    fn print_torsions(self);
//}

