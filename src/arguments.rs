// exit function, Used to disregard panic!() messages
use std::process::exit;

pub fn return_cli_arguments(cli_args: Vec<String>) -> Flags {

    // If help is prompted
    if &cli_args[1] == "-h" || &cli_args[1] == "--help" { print_help() }

    // Only one argument allowed
    if cli_args.len() < 3 { 
        println!("Not enough arguments prompted!");
        exit(1)
    }

    // Instantiate a Flags struct
    let mut flag = Flags::new();

    // Loop over the `args` argument, which are cli_arguments
    for (i, arg) in cli_args.iter().enumerate() {
        if arg == "--peptide" { 
            flag.add_torsion_and_num_fields(i, TorsionType::Peptide, &cli_args)
        };
        if arg == "--fivering" { 
            flag.add_torsion_and_num_fields(i, TorsionType::Fivering, &cli_args)
        };
        if arg == "--sixring" { 
            flag.add_torsion_and_num_fields(i, TorsionType::Sixring, &cli_args)
        };

        if arg == "--twopi" { flag.twopi = true };
        if arg == "--rad" { flag.rad = true };

    }
    // return the Flags struct
    flag
}


// Define Flags struct
#[derive(Debug)]
pub struct Flags {
    pub torsion_type : TorsionType,
    pub rad : bool,
    pub num : u32,
    pub twopi : bool 
}

#[derive(Debug)]
pub enum TorsionType {
    Peptide,
    Fivering,
    Sixring,
    Nothing
}
// Implemented amount of methods : 2
impl Flags {
    fn new() -> Flags{ //torsions : String, intv : usize
        Flags {
            torsion_type : TorsionType::Nothing, // converts &str type to String type
            rad : false,
            twopi : false,
            num : 0
        }
    
    }

    fn add_torsion_and_num_fields(&mut self, idx : usize, torsion : TorsionType, args : &Vec<String>) {
        // take ownership of the argument value and take it
        self.torsion_type = torsion;

        // Retrieve the next value in the Vec<> and check if it is parsable as a `usize`
        self.num = match args[idx + 1].parse::<u32>() {
            Ok(integer) => integer,
            Err(_) => num_not_prompted_correctly()
        }

    }
}



#[allow(unreachable_code)]
fn num_not_prompted_correctly() -> u32 {
    println!("Prompted NUM is not an integer value or was not prompted at all.");
    exit(1);
    0 // Hacky shit to make the matching arm not complain

}

fn print_help() {
    println!(
        "Torsions help menu :
            --peptide NUM : to generate torsion angles for peptide-like systems
            --fivering NUM : to generate torsion angles for five-membered ring systems
            --sixring NUM : to generate torsion angles for six-membered ring systems
            \n
            --twopi : to convert torsion angles from [-180,180] (default) to [0,360]
            --rad : to convert torsion angles from degrees (default) to radians
            \n
            -h or --help to print this menu. "
        );
        exit(0)

}
