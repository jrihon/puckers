/// Since we work it large array sizes, depending on the query sizes, 
/// We will have to work with more manageable data-wise and working on such large array sizes will
/// make the code look more complicated, with more indented loops and indexing.
///
/// That is why, with the information of the ring partition struct, we will rebuild every spherical
/// coordinate separately in a function
/// Essentially, we will design a struct that will hold several method to make code more legible.
/// Then, in `mod.rs` , we will have a Vec<Struct> that makes it far more manageable to hold all
/// the data
///
///

/// We prepare for the 3D coordinates , with each two first coordinates being filled in for the
/// time being and the z-coordinate remaining 0.0 for a while.
struct AtomPosition {
    s11 : [f64; 3],
    s12 : [f64; 3],
    s13 : [f64; 3],
    s21 : [f64; 3],
    s22 : [f64; 3],
    s23 : [f64; 3],
    s31 : [f64; 3],
    s32 : [f64; 3],
    s33 : [f64; 3],
}

struct MolReconstruction {
}

pub fn reconstruct_coordinates() {
    todo!()
    
    // let pO = [f64; 3];
    // let pP = [f64; 3];
    // let pQ = [f64; 3];
}
