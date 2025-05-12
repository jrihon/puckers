use crate::torsion_typing::TorsionType;
use std::slice::Iter;
use std::{panic, println, process::exit};

/// Cli arguments struct
#[derive(Debug)]
pub struct Flags {
    pub torsion_type: Option<TorsionType>,
    pub num: u64,
}

impl Flags {
    /// Initialise Flags::new() -> Flags
    /// ```
    /// Flags {
    ///     torsion_type : None,
    ///     num : 0
    /// }
    /// ```
    pub fn new() -> Flags {
        Flags {
            torsion_type: None,
            num: 0,
        }
    }

    /// add Torsion type to the Flags Struct
    /// add num amount to the Flags Struct
    fn define_torsion_type(&mut self, torsion: TorsionType, iter: &mut Iter<'_, String>) -> () {
        self.torsion_type = Some(torsion);

        self.num = match iter.next() {
            Some(a) => match a.parse::<u64>() {
                Ok(num) => num,
                Err(_) => panic!("`num` not parsed as integer... Aborting."),
            },
            None => panic!("End of query, no `num` value prompted."),
        }
    }

    /// Pass the Vec of Strings, which are the CLI arguments that are given to puckers
    /// and are processed and returned as a neat struct to the main function.
    pub fn return_cli_arguments(cli_args: Vec<String>) -> Flags {
        // If help is prompted
        if cli_args.len() == 1 {
            print_help()
        };

        // Instantiate a mutable Flags struct
        let mut flag = Flags::new();

        let mut cli_iter = cli_args.iter();

        loop {
            // while the iterator produces valid Some(x) types

            match cli_iter.next() {
                Some(cli) => {
                    match &cli[..] {
                        // from String to &str type
                        "--peptide" => {
                            // saveguard if two torsion types have been queried,
                            // only first one matters
                            if flag.torsion_type.is_none() {
                                flag.define_torsion_type(TorsionType::Peptide, &mut cli_iter)
                            }
                        }
                        "--fivering" => {
                            if flag.torsion_type.is_none() {
                                flag.define_torsion_type(TorsionType::Fivering, &mut cli_iter)
                            }
                        }
                        "--sixring" => {
                            if flag.torsion_type.is_none() {
                                flag.define_torsion_type(TorsionType::Sixring, &mut cli_iter)
                            }
                        }
                        "-h" | "--help" => print_help(),
                        _ => (), // exhaustive match; do nothing.
                    }
                }
                // end of cli argument query
                None => break,
            }
        }

        if flag.torsion_type.is_none() {
            panic!("No `torsion type` queried... Aborting.")
        }

        flag
    }
}

fn print_help() {
    println!(
        "Pucke.rs help menu :
       --peptide  NUM : to generate torsion angles for peptide-like systems
       --fivering NUM : to generate torsion angles for five-membered ring systems
       --sixring  NUM : to generate torsion angles for six-membered ring systems
       -h or --help   : to print this menu. "
    );
    exit(0)
}
