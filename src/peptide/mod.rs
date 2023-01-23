// External crate
use ndarray::prelude::*;
use std::clone::Clone;

// This is how we import.
// We start from our own crate and pass the absolute path to the `use` keyword
use crate::arguments::Flags;
use crate::Torsions;

pub fn peptide(flags: Flags) -> Torsions {
    // let the variable torsions return as a set of dihedrals
    // State at which range we want to generate the arrays
    let range1 = if flags.twopi {
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
        }.generate_dihedrals(); // Consume the Dihedrals struct and return a 1D array, the X axis
                                
    let range2 = range1.clone(); // Clone the range for the Y axis

    generate_x_and_y_axis(flags.num, range1, range2) // returned value
}

fn generate_x_and_y_axis(num : u32, r1 : Array1<f64>, r2 : Array1<f64>) -> Torsions {

    let _sizeof : u32 = num * num;
    let num_f64 : f64 = num as f64;
    
    let mut t = Torsions::new(_sizeof as usize);

    let mut x : f64;
    let mut y : f64;
    for i in 0.._sizeof {
        // Calculate indexes for the array axises
        x = (i as f64 / num_f64).floor(); // X axis, returns with floor
        y = i as f64 % num_f64; // Y axis, return with modulo

        // fill out the array
        t.array1[i as usize] = r1[x as usize]; // indexing is only allowed with usize types
        t.array2[i as usize] = r2[y as usize]; // indexing is only allowed with usize types

    }

    t // return Torsions struct
}

// Create struct to generate your desired ranges of dihedrals
struct Dihedrals {
    start : f64,
    end : f64,
    num : u32
}

// Implement a method of the Dihedrals struct
impl Dihedrals {
    fn generate_dihedrals(&self) -> Array1<f64> {
        Array1::linspace(self.start, self.end, self.num as usize)
    }
}

// Implement the Copy trait
impl Copy for Dihedrals {}

// Implement the Clone trait to apply X.clone() on one of our structs
impl Clone for Dihedrals {
    fn clone(&self) -> Self {
        *self
    }

}
