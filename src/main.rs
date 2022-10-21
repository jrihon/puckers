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


// Declare custom modules in main.rs
mod arguments;
mod peptide;

use arguments::return_cli_arguments;

// MAIN
fn main() {

    let mut _args: Vec<String> = args().collect();

    let cli_arguments = return_cli_arguments(_args);
    match cli_arguments.torsion_type.as_str() {
        "--peptide" =>  peptide::peptide(&cli_arguments),
        "--fivering" =>  fivering(),
        "--sixring" =>  sixring(),
        _ => panic!("Flag Not Found")
    };
}



fn fivering() { println!("Zx and Zy") }
fn sixring() { println!("equidistant globe") }
