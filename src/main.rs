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

use arguments::return_cli_arguments;

// MAIN
fn main() {

    let mut _args: Vec<String> = args().collect(); // collect passed arguments from CLI

    let cli_arguments = return_cli_arguments(_args); // return CLI arguments in a convenient Struct

    // Match the type of torsion angles needed to generate and then output them
    let torsions = match cli_arguments.torsion_type.as_str() {
        "--peptide" =>  peptide::peptide(&cli_arguments),
        "--fivering" =>  fivering::fivering(&cli_arguments),
        "--sixring" =>  sixring:: sixring(),
        _ => panic!("Flag Not Found")
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
    fn print_arrays(&self) {
        let _sizeof : usize = self.array1.len(); // define size of array

        // print both arrays
        for i in 0.._sizeof {
            println!("{:?}", (self.array1[i], self.array2[i]))

        };
    }
}
