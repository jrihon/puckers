/// A program, written in the greatest language of all, to generate dihedral values
/// in order to perform conformational sampling on : 
///     peptide-like molecules (or any set of two torsion angles),
///     five-membered furanose rings,
///     six-membered pyranose rings.
///
///
/// License   : MIT License
/// Author    : Jérôme Rihon
/// Institute : Rega Institute for Medical Research, Katholieke Universiteit Leuven.
///             Leuven 3000, Belgium.
///
///
// Standard libs
use std::env::args;
use ndarray::Array1;


// Declare custom modules in main.rs
mod arguments;
mod peptide;
mod fivering;
mod sixring;

use crate::arguments::{return_cli_arguments, TorsionType};

// MAIN
fn main() {

    let args: Vec<String> = args().collect(); // collect passed arguments from CLI

    // We can actually use the clap crate, but I wanted to do it manually to get a feel for Rust
    let cli_arguments = return_cli_arguments(args); // return CLI arguments in a convenient Struct

    // Match the type of torsion angles needed to generate and then output them
    //
    // match against a reference
    // if we match against the value, we move the value out of the torsion_type field in the struct
    // and then we lose it and get a partial move (where one of the field's values has been moved
    // in a struct)
    // That is why we match against a reference to the value
    let torsions = match &cli_arguments.torsion_type {
        Some(a) => match a {
            TorsionType::Peptide =>  peptide::peptide(cli_arguments),
            TorsionType::Fivering =>  fivering::fivering(cli_arguments),
            TorsionType::Sixring =>  sixring:: sixring(cli_arguments),
        },
        None => panic!("Flag Not Found")
    };

    // Print resulting arrays
    torsions.print_arrays();
}


// struct that contains the required torsion angles, be it for any type of system,
// and writes it out to a file
pub struct Torsions {
    pub array1 : Array1<f64>,
    pub array2 : Array1<f64>
}

impl Torsions {
    fn new(num : usize) -> Torsions {
        Torsions {
           array1 : Array1::<f64>::zeros(num),
           array2 : Array1::<f64>::zeros(num),
    }
        
    }
    fn print_arrays(&self) {
        let size : usize = self.array1.len(); // define size of array

        // print both arrays
        for i in 0..size {
            println!("{:?}", (self.array1[i], self.array2[i]))

        };
    }
}
