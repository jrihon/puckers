// External crate
use ndarray::prelude::*;



pub fn peptide() -> () {
    let torsions = Dihedrals {
        start : 0.,
        end : 360.,
        interval : 19
    }.generate_dihedrals();

    println!("{}", torsions)
}


// Create struct to generate your desired ranges of dihedrals
struct Dihedrals {
    start : f32,
    end : f32,
    interval : usize
}

// Implement a method of the Dihedrals struct
impl Dihedrals {
    fn generate_dihedrals(&self) -> Array1<f32> {
        Array1::linspace(self.start, self.end, self.interval)
    }
}

