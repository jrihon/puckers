/// "How to generate equidistributed points on the surface of a sphere"
///                 Mark Deserno, 2004, Max Planck Institute
///
///
///     x = rho . sin(theta) . cos(phi)
///     y = rho . sin(theta) . sin(phi)
///     z = rho . cos(phi)
///
///     where rho is a constant value, e.g. the radius of the sphere
///
///
///
///     Regular equidistribution can be achieved by choosing circles of latitude
///     at constant intervals `d_theta`, where on every circle the distance between
///     the points is `d_phi`. 
///     The values are chosen such that `d_theta` is roughly equal to`d_phi` and
///      `d_theta` times `d_phi` gives the average points per point.
///
///
///     4 * PI * r^2 = surface area of a sphere
///

// import modules
use std::f64::consts::PI;
use ndarray::Array1;

const RHO : f64 = 0.67; // radius of the sphere; constant
pub const TWOPI : f64 = 2. * PI; // two pi; constant

pub fn equidistance_globe(num : u32 ) -> GlobeCoordinates {
    // Set a value as surface area / points
    let corrected_num: f64 = corrected_amount_of_points(num as f64);
    let a: f64 = ( 4.*PI*RHO.powi(2)) / corrected_num;

    let mut idx : u32 = 0; // indexing the arrays

    // Set d as the square root of a
    let d: f64 = a.sqrt();

    // Round of the ratio between PI and the value of d
    let m_theta: f64 = (PI / d).round();

    // Set d_theta and d_phi
    let d_theta: f64 = PI / m_theta;
    let d_phi: f64 = a / d_theta;

    let num_sizeof: usize = corrected_num_amount_to_size_up_arrays(m_theta, d_phi);
    // Instance struct
    let mut globe = GlobeCoordinates::new(num_sizeof, m_theta as usize);

    for m in 0..m_theta as u32 {
        globe.theta[m as usize] = (PI * (m as f64 + 0.5)) / m_theta;
        let m_phi : f64 = (TWOPI * globe.theta[m as usize].sin() / d_phi).round();

        for n in 0..m_phi as u32 {
            globe.phi[idx as usize] = (TWOPI * n as f64) / m_phi;
            globe.polar_to_cartesian(idx as usize, m as usize);
            idx = idx + 1;
            
        }
    }

    globe // return the struct containing all the points on the surface of our globe
}


fn corrected_num_amount_to_size_up_arrays(m_theta : f64, d_phi : f64) -> usize {
    // Counting the amount of points that are actually generated
    let mut size_array: u32 = 0;

    for m in 0..m_theta as u32 {
        let theta: f64 = (PI * (m as f64 + 0.5)) / m_theta;
        let m_phi : f64 = (TWOPI * theta.sin() / d_phi).round();
        size_array = size_array + m_phi as u32;

    };

    size_array as usize // return exact amount of points that will have been generated
}

fn corrected_amount_of_points(num : f64) -> f64 {
    // Markus Deserno's mathematics only works out if we commit to a radius = 1 unit
    // 
    // Since the Rho value (the radius of the sphere) is set to be 0.67 for our purposes (see
    // Cremer-Pople standard puckering values), we need to correct the amount of prompted points.
    // 
    // What we need is the ratio of the surface are at rho(0.67) and rho(1.00)
    // --> (0.67^2 * PI * 4) / (1.00^2 * PI * 4) => 0.67^2
    num * RHO.powi(2)
}


impl GlobeCoordinates {
    fn new(num: usize, m_theta : usize) -> GlobeCoordinates {
        GlobeCoordinates {
        x : Array1::<f64>::zeros(num),
        y : Array1::<f64>::zeros(num),
        z : Array1::<f64>::zeros(num),
        rho : RHO,
        theta : Array1::<f64>::zeros(m_theta),
        phi : Array1::<f64>::zeros(num),
        }
    }

    fn polar_to_cartesian(&mut self, i : usize, m : usize) {
        self.x[i] = self.rho * self.theta[m].sin() * self.phi[i].cos();
        self.y[i] = self.rho * self.theta[m].sin() * self.phi[i].sin();
        self.z[i] = self.rho * self.theta[m].cos();     
    }
}


pub struct GlobeCoordinates {
    pub x : Array1<f64>,
    pub y : Array1<f64>,
    pub z : Array1<f64>,
    pub rho : f64,
    pub theta : Array1<f64>,
    pub phi : Array1<f64>,
}
