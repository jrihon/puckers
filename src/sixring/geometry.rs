#![allow(unused_imports)]
/// General Linear Algebra and math stuff I do not want to import from different libraries
/// so I write it myself and implement on my own types
///

use std::f64::consts::PI;

/// 3D coordinates-types and -matrices from primitives
pub type Coordinate = [f64; 3];
pub type DirectionAxis = [f64; 3];
pub type RotationMatrix = [[f64; 3]; 3];


///
///
///
///
///
///
///
///
///
/// Implement standard linear algebra on the Coordinate types
pub trait LinAlg {
    fn dot_product(&self, rhs : &Coordinate) -> f64;
    fn cross_product(&self, rhs : &Coordinate) -> DirectionAxis;
    fn subtract_arr(&self, rhs : &Coordinate) -> Coordinate;
    fn normalise_vector(&self) -> Coordinate;
    fn norm(&self) -> f64;
    fn scale_vector(&self, factor: f64) -> Coordinate;

}


impl LinAlg for Coordinate {

    /// Calculate the scalar product between two vectors
    fn dot_product(&self, rhs : &Coordinate) -> f64 {
      (self[0] * rhs[0]) + (self[1] * rhs[1]) + (self[2] * rhs[2])
    }

    /// Calculate the cross product between two vectors
    fn cross_product(&self, rhs : &Coordinate) -> DirectionAxis {

        [
            (self[1] * rhs[2]) - (self[2]) * (rhs[1]),
           (-self[0] * rhs[2]) + (self[2]) * (rhs[0]),
            (self[0] * rhs[1]) - (self[1]) * (rhs[0]),
        ]

    }

    /// Subtract one Coordinate from another
    fn subtract_arr(&self, rhs : &Coordinate) -> Coordinate {
        [ self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2] ]
    }

    /// -> sqrt(x² + y² + z²)
    /// This gives the length of the vector
    fn norm(&self) -> f64 {
        self.map(|x: f64| x.powi(2))
            .into_iter()
            .sum::<f64>()
            .sqrt()
    }

    /// Normalise the size of the Coordinate
    fn normalise_vector(&self) -> Coordinate {
       let d = 1. / self.norm(); 

       self.map(|x: f64| d * x) // apply the factor `d` to all elements of the coordinate
    }

    /// Scale the vector by a certain factor
    fn scale_vector(&self, factor: f64) -> Coordinate {
        self.map(|x| x * factor)
    }


}



///
///
///
///
///
///
///
///
/// Provide standard linear algebra operations also as
/// standalone function
pub fn dot_product(a : Coordinate, b : Coordinate) -> f64 {
    (a[0] * b[0]) + (a[1] * b[1]) + (a[2] * b[2])

}

pub fn cross_product(a : Coordinate, b : Coordinate) -> DirectionAxis {

    [
        (a[1] * b[2]) - (a[2]) * (b[1]),
       (-a[0] * b[2]) + (a[2]) * (b[0]),
        (a[0] * b[1]) - (a[1]) * (b[0]),
    ]

}

/// -> sqrt(x² + y² + z²)
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

/// Calculate the dihedral between four Coordinate points
/// A dihedral is an angle between four points.
/// Essentially : 
///     get three vector from the four points; b0, b1 and b2
///     from cross(b0, b1) and cross(b1, b2) we get two direction axes
///     -> The dot product between those to direction axes results in the dihedral angle
///
///     Here we use the praxeolitic formula, which involves 1 sqrt and 1 cross product
///     This does not use the description above, but it is more performant than this description
///     See : https://stackoverflow.com/questions/20305272/dihedral-torsion-angle-from-four-points-in-cartesian-coordinates-in-python
///
/// Semantically, it makes little sense to implement the dihedral function as a method on
/// the Coordinate type, as we need to add three more Coordinate variables to the function
/// arguments
/// That is why it remains as a standalone public function
/// TODO : This function needs excessive testing !
/// TODO : Create mod test and test out several known sets of four points with known dihedrals !
/// TODO : Do this before continuing to write other functions !
///
//pub fn dihedral() -> f64 {
pub fn dihedral(p0 : Coordinate, p1 : Coordinate, p2 : Coordinate, p3 : Coordinate) -> f64 {
    let b0 = p0.subtract_arr(&p1);
    let b1 = p2.subtract_arr(&p1).normalise_vector();
    let b2 = p3.subtract_arr(&p2);
    
    // Vector rejections (as opposed to projections)
    // the b0/b2 will receive a rhs-subtraction from the b1-vector, which has been scaled
    let w = b0.subtract_arr( 
          &b1.scale_vector( 
              b0.dot_product(&b1)
          )
    );
    let v = b2.subtract_arr( 
          &b1.scale_vector( 
              b2.dot_product(&b1)
          )
    );
    
    
    let x = v.dot_product(&w);
    let y = b1.cross_product(&v).dot_product(&w);
    
    // return in degrees : 
    y.atan2(x) * (180. / PI)
    
    
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

    /// apply a Rotation to a Coordinate, with a specified axis of the RotationMatrix
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
