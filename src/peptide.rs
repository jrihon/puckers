// External crate
use ndarray::prelude::*;

// This is how we import.
// We start from our own crate and pass the absolute path to the `use` keyword
use crate::arguments::Flags;

pub fn peptide(flags: &Flags) -> () {
    // let the variable torsions return as a set of dihedrals
    // State at which range we want to generate the arrays
    let torsions = if flags.twopi {
        Dihedrals {
            start : 0.,
            end : 360.,
            num : flags.num
        }
    } else {
        Dihedrals {
            start : -180.,
            end : 180.,
            num : flags.num
            }
        }.generate_dihedrals(); // Consume the Dihedrals struct and return a 1D array

    println!("{}", torsions)
}


// Create struct to generate your desired ranges of dihedrals
struct Dihedrals {
    start : f32,
    end : f32,
    num : usize
}

// Implement a method of the Dihedrals struct
impl Dihedrals {
    fn generate_dihedrals(&self) -> Array1<f32> {
        Array1::linspace(self.start, self.end, self.num)
    }
}

