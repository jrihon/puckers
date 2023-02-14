/// A program, written in the greatest language of all, to generate dihedral values
/// in order to perform conformational sampling on : 
///     peptide-like molecules (or any set of two torsion angles),
///     five-membered/furanose rings,
///     six-membered/pyranose rings.
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

// Declare modules
mod arguments;
mod torsion_typing;
mod peptide;
mod fivering;
mod sixring;

// Use own libs
use crate::arguments::Flags;
use crate::torsion_typing::{TorsionType, Dihedrals};

///
///
///
///
///
///
///
/// ---------------- MAIN ---------------- 
#[allow(unused_variables)]
fn main() {

    let args: Vec<String> = args().collect(); // collect passed arguments from CLI

    // We can actually use the clap crate, but I wanted to do it manually to get a feel for Rust
    let flags = Flags::return_cli_arguments(args); // return CLI arguments in a convenient Struct

    // get the torsion angles and the axes
    let puckers = run(&flags);

    // Print the results
    puckers.print_to_stdout(flags);
}



fn run(flags :&Flags) -> Box<dyn Dihedrals + 'static> {

    // Match the type of torsion angles needed to generate and then output them
    //
    // match against a reference
    // if we match against the value, we move the value out of the torsion_type field in the struct
    // and then we lose it and get a partial move (where one of the field's values has been moved
    // in a struct)
    // That is why we match against a reference to the value
    let t = match &flags.torsion_type {
        Some(torsion) => match torsion {
            TorsionType::Backbone =>  peptide::peptide(flags),
            TorsionType::Fivering =>  fivering::fivering(flags),
            TorsionType::Sixring =>  sixring:: sixring(flags),
        },
        None => panic!("Flag Not Found")
    };
    t
}
