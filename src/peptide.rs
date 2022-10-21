// External crate
use ndarray::prelude::*;

// This is how we import.
// We start from our own crate and pass the absolute path to the `use` keyword
use crate::arguments::Flags;

pub fn peptide(flags: &Flags) -> () {
    let torsions = Dihedrals {
        start : -180.,
        end : 180.,
        interval : flags.interval
    }.generate_dihedrals();

    println!("{}", torsions)
}


// Create struct to generate your desired ranges of dihedrals
struct Dihedrals {
    start : f32,
    end : f32,
    interval : usize
}

// Implement a method of the Dihedrals struct
impl Dihedrals {
    fn generate_dihedrals(&self) -> Array1<f32> {
        Array1::linspace(self.start, self.end, self.interval)
    }
}

