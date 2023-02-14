use std::process::exit; // exit function, Used to disregard panic!() messages

use crate::torsion_typing::TorsionType;

/// How cli arguments
//#[derive(Debug, Clone)]
pub struct Flags {
    pub torsion_type : Option<TorsionType>,
    pub num : u64,
    pub rad : bool,
    pub axis : bool,
//    pub twopi : bool,
}

// Implemented amount of methods : 2
impl Flags {
    fn new() -> Flags{ 
        Flags {
            torsion_type : None, // converts &str type to String type
            rad : false,
            axis : false,
//            twopi : false,
            num : 0
        }
    
    }

    /// Parse type of torsional angle and amount of points to sample
    fn add_torsion_and_num_fields(&mut self, idx : usize, torsion : Option<TorsionType>, args : &[String]) {
        // take ownership of the argument value and take it
        self.torsion_type = torsion ;

        // Retrieve the next value (after the torsiontype argument) and check if it is parsable as a `usize`
        self.num = match args[idx + 1].parse::<u64>() {
            Ok(integer) => integer.to_owned(),
            Err(_) => num_not_prompted_correctly()
        }

    }

    /// Pass the Vec of Strings, which are the CLI arguments that are given to puckers
    /// and are processed and returned as a neat struct to the main function.
    pub fn return_cli_arguments(mut cli_args: Vec<String>) -> Flags {

        // If help is prompted
        if cli_args.len() == 1 { print_help() };

        // If help is prompted
        if &cli_args[1] == "-h" || &cli_args[1] == "--help" { print_help() };

        // Only one argument allowed
        if cli_args.len() < 3 { 
            println!("Not enough arguments prompted!");
            exit(1)
        };

        // Instantiate a mutable Flags struct
        let mut flag = Flags::new();

        cli_args.drain(..1); // remove $PATH to binary as one of the cli args

        // Loop over the `args` argument, which are cli_arguments
        for (i, arg) in cli_args.iter().enumerate() {
            if arg == "--peptide" { 
                flag.add_torsion_and_num_fields(i, Some(TorsionType::Backbone), &cli_args)
            } else if arg == "--fivering" { 
                flag.add_torsion_and_num_fields(i, Some(TorsionType::Fivering), &cli_args)
            } else if arg == "--sixring" { 
                flag.add_torsion_and_num_fields(i, Some(TorsionType::Sixring), &cli_args)
            } else if arg == "--rad" {
                flag.rad = true 
            } else if arg == "--axis" {
                flag.axis = true 
//            } else if arg == "--twopi" {
//                flag.twopi = true 
            } else if arg.parse::<u64>().is_ok() {
            } else {
                panic!("`{}` is not a valid argument.", arg)
            }

        };


        flag
    }
    
}



#[allow(unreachable_code)]
fn num_not_prompted_correctly() -> u64 {
    println!("Prompted NUM is not an integer value or was not prompted at all.");
    exit(1);
    0 // Hacky sh0t to make the matching arm not complain

}

fn print_help() {
    println!(
        "Puckers help menu :
       --peptide NUM : to generate torsion angles for peptide-like systems
       --fivering NUM : to generate torsion angles for five-membered ring systems
       --sixring NUM : to generate torsion angles for six-membered ring systems
       \n
       --rad : to convert torsion angles from degrees (default) to radians
       --axis : print out axes alongside the torsion angles
       \n
       -h or --help to print this menu. "
        );
        exit(0)

}
