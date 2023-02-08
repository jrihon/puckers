
use ndarray::Array1;

// Use own libs
use crate::arguments::Flags;
use crate::torsion_typing:: Peptide;

pub fn peptide(flags: Flags) -> Peptide {
    // let the variable torsions return as a set of dihedrals
    // State at which range we want to generate the arrays
    let range = if flags.twopi {
        [0., 360.]
    } else {
        [-180., 180.]
    }; 

    Peptide::new(flags.num, range[0], range[1])
                                
}


impl Peptide {

    /// Instantiate the entire range of the phi-psi backbone dihedrals
    pub fn new(num : u64, start: f64, end: f64) -> Peptide {

        let _sizeof : u64 = num * num;

        let ax1 = Array1::linspace(start, end, num as usize);
        let ax2 = ax1.clone();

        let mut p = Peptide{
            phi : Array1::zeros(_sizeof as usize),
            psi : Array1::zeros(_sizeof as usize),
        };
        
        let mut x : f64;
        let mut y : f64;
        for i in 0.._sizeof {

            // For every x value, return all y values
            x = (i as f64 / num as f64).floor(); // floor, to return x axis value
            y = i as f64 % num as f64; // return with modulo, to return y axis value

            // fill out the array
            p.phi[i as usize] = ax1[x as usize]; 
            p.psi[i as usize] = ax2[y as usize]; 

        }

        p 
    }

}
