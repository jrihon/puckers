// import module(sixring) modules
mod equidistance_sphere;
mod geometry;
mod local_elevation;
mod reconstruct_ring;
mod ring_partition;

use crate::sixring::equidistance_sphere::equidistance_sphere;

use crate::arguments::Flags;
use crate::torsion_typing:: Pyranose;
use crate::sixring::ring_partition::RingPartition;

use geometry::dihedral;

const PI_DEG : f64 = 180.;



/// Calculate possible sampling space (spherical coordinates)
/// and 
pub fn sixring(flags: Flags) -> Pyranose {

    let sphere = equidistance_sphere(flags.num);

    let projection = local_elevation::cremerpople_evelation(&sphere)
                                        .projection_and_partition(sphere.amount);

    let mut p = Pyranose::new(sphere.amount);

    let vec_of_pyranoses = reconstruct_ring::reconstruct_coordinates(
                            &projection,
                            sphere.amount,
                            local_elevation::cremerpople_evelation(&sphere), // Zj matrix
    );

    for (i, pyr) in vec_of_pyranoses.iter().enumerate() {
        p.alpha1[i] = dihedral(pyr.p5, pyr.p1, pyr.p3, pyr.p2) - PI_DEG;
        p.alpha2[i] = dihedral(pyr.p1, pyr.p3, pyr.p5, pyr.p4) - PI_DEG;
        p.alpha3[i] = dihedral(pyr.p3, pyr.p5, pyr.p1, pyr.p6) - PI_DEG;
    }

    p
}

//fn print_sphere_cartesians(sphere: GlobeCoordinates) {
//    for i in 0..sphere.x.len() {
//        println!("{:?}", (sphere.x[i], sphere.y[i], sphere.z[i]))
//    }
//}
//
//fn print_sphere_sphericals(sphere: GlobeCoordinates) {
//    let mut idx_theta: usize = 0;
//
//    for i in 0..sphere.phi.len() {
//        // every new circle, we start off at phi == 0.0 rad
//        if (sphere.phi[i] == 0.0) && i != 0 {
//            idx_theta += 1
//        };
//
//        println!("{:?}", (sphere.rho, sphere.theta[idx_theta], sphere.phi[i]))
//    }
//}
