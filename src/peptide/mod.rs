//use ndarray::Array1;

// Use own libs
use crate::arguments::Flags;
use crate::torsion_typing::{Peptide, BackboneCoordinates, Dihedrals, Axis};



pub fn peptide(flags: &Flags) ->(Box<dyn Dihedrals>, Box<dyn Axis>) {
//pub fn peptide(flags: Flags) -> (Box<&'static Peptide>, Box<&'static BackboneCoordinates>) {
    // let the variable torsions return as a set of dihedrals
    // State at which range we want to generate the arrays
    let range = if flags.twopi {
        [0., 360.]
    } else {
        [-180., 180.]
    }; 

    let _sizeof : u64 = flags.num * flags.num;

    let bb = BackboneCoordinates::new(range[0], range[1], flags.num as usize);

    let mut p = Peptide::new(_sizeof as usize);
    
    let mut x : f64;
    let mut y : f64;
    for i in 0.._sizeof as usize {

        // For every x value, return all y values
        x = (i as f64 / flags.num as f64).floor(); // floor, to return x axis value
        y = i as f64 % flags.num as f64; // return with modulo, to return y axis value

        // fill out the array
        p.phi[i as usize] = bb.x[x as usize]; 
        p.psi[i as usize] = bb.y[y as usize]; 
    }

    (Box::new(p), Box::new(bb))
}
