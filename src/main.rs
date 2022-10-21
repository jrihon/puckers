/// Make a flag to allow the user to define range from -pi to pi || 0 to 2pi
///     like --2pi to explicitly define this and default to the other one
///
/// Make a flag to allow the returned arrays to be printed in radians
///     like --rad , this program defaults with printing to degrees
///
/// After the standard flag `peptide, fivering, sixring` add an interval value, which 
///     should be parsed as a usize value
///

// Standard libs
use std::env::args;


// Custom libs
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
