use crate::arguments::Flags;
use crate::torsion_typing::{Peptide, PeptideAxes, Dihedrals};

/// Generate the torsion angles to use as restraints for peptide-like molecules
pub fn peptide(flags: &Flags) -> Peptide {

    let amount : u64 = flags.num * flags.num;

    let bb = PeptideAxes::new(flags.num as usize);

    let mut p = Peptide::new(amount as usize);
    
    let mut x : f64;
    let mut y : f64;
    for i in 0..amount as usize {

        // For every x value, return all y values
        x = (i as f64 / flags.num as f64).floor(); // floor, to return x axis value
        y = i as f64 % flags.num as f64; // return with modulo, to return y axis value

        // fill out the array
        p.phi[i as usize] = bb.x[x as usize]; 
        p.psi[i as usize] = bb.y[y as usize]; 
    }

    // Values are ORCA-ready
    p
}
