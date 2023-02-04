/// General Linear Algebra and math stuff I do not want to import from different libraries
/// so I write it myself
///

/// 3D coordinates-types and -matrices from primitives
pub type Coordinate = [f64; 3];
pub type DirectionAxis = [f64; 3];
pub type RotationMatrix = [[f64; 3]; 3];




/// Calculate the scalar product between two vectors
pub fn dot_product(a : Coordinate, b : Coordinate) -> f64 {
  (a[0] * b[0]) + (a[1] * b[1]) + (a[2] * b[2])

}

/// Calculate the direction axis between two vectors
pub fn direction_axis(a : Coordinate, b : Coordinate) -> DirectionAxis {

    [
        (a[1] * b[2]) - (a[2]) * (b[1]),
       (-a[0] * b[2]) + (a[2]) * (b[0]),
        (a[0] * b[1]) - (a[1]) * (b[0]),
    ]

}

/// Map over every coordinate of the vector
/// to the power of `2`, sum it all up and take the squareroot() of that value
/// This gives the length of the vector
fn norm(c : &Coordinate) -> f64 {
    c.map(|x: f64| x.powi(2))
        .into_iter()
        .sum::<f64>()
        .sqrt()
}

/// Normalise the vector, often used as a directional axis
pub fn normalise_vector(c : Coordinate) -> Coordinate {
   let d = 1. / norm(&c); 

   c.map(|x: f64| d * x) // apply the factor `d` to all elements of the coordinate
}





/// Custom trait to extend primitive type :
/// Make extension trait on the primitive type `RotationMatrix`
pub trait RotMatrix {
    fn new(phi: f64) -> RotationMatrix; 
    fn apply_rotation(&self, p : Coordinate) -> Coordinate; 
    fn apply_rotation_around_g(&self, p : Coordinate, idx: usize) -> f64; 
}

impl RotMatrix for RotationMatrix {

    /// make a RotationMatrix out of an angle `phi`
    fn new(phi: f64) -> RotationMatrix {

        [ [ phi.cos(), -phi.sin(), 0., ],   // rotation around i-hat (x-axis)
          [ phi.sin(),  phi.cos(), 0., ],   // rotation around j-hat (y-axis)
          [        0.,         0., 1.  ] ]  // rotation around k-hat (z-axis)
        
    }

    /// apply a Rotation to a Coordinate
    fn apply_rotation(&self, p : Coordinate) -> Coordinate {
        let mut c = [0.,0.,0.];

        for (i, arr) in self.iter().enumerate() { 
            c[i] = (arr[0] * p[0]) + (arr[1] * p[1]) + (arr[2] * p[2])
        }

        c
    }

    /// apply a Rotation to a Coordinate, with a specified axis
    fn apply_rotation_around_g(&self, p : Coordinate, idx: usize) -> f64 {
            (self[idx][0] * p[0]) + (self[idx][1] * p[1]) + (self[idx][2] * p[2])
        }
}

/// Since we can't perform operator overloading on primitive types (because we do not own them, see Rust's Orphan Rule)
/// , we have to make do with a small function to allow for operations on array-types
///
/// Wrapping the array-type in a struct and then doing overloading on that struct
/// would makes all the other uses of the Coordinate type in this program very ugly
pub fn subtract_arr(a : Coordinate, b : Coordinate) -> Coordinate {
    [ a[0] - b[0], a[1] - b[1], a[2] - b[2] ]
}
