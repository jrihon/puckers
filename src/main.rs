/// A program, written in the greatest language of all, to generate dihedral values
/// in order to perform conformational sampling on : 
///     peptide-like molecules (or any set of two torsion angles),
///     five-membered/furanose rings,
///     six-membered/pyranose rings.
///
///
/// License   : GPL2 License
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
fn main() -> () {

    // Disregard Clap, transcend humanity
    let flags = Flags::return_cli_arguments( args().collect() ); // collect CLI arguments and parse I/O

    // get the torsion angles and print out
    run(flags);
}



fn run(flags :Flags) -> () {

    // Match the type of torsion angles needed to generate and then output them
    match flags.torsion_type.as_ref().unwrap() { // `as_ref()` because we consume we otherwise consume the Enum when matching
        TorsionType::Peptide =>  peptide::peptide(&flags).print_values(flags),
        TorsionType::Fivering => fivering::fivering(&flags).print_values(flags),
        TorsionType::Sixring =>  sixring::sixring(&flags).print_values(flags),
    }
}
