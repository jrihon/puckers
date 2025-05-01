// import module(sixring) modules
pub mod equidistance_sphere;
mod geometry;
mod local_elevation;
mod reconstruct_ring;
mod ring_partition;

use crate::sixring::equidistance_sphere::equidistance_sphere;

use crate::arguments::Flags;
use crate::sixring::ring_partition::RingPartition;
use crate::torsion_typing::Pyranose;

use geometry::dihedral;

/// Calculate possible sampling space (spherical coordinates)
pub fn sixring(flags: &Flags) -> Pyranose {
    let sphere = equidistance_sphere(flags.num);

    let projection =
        local_elevation::cremerpople_evelation(&sphere).projection_and_partition(sphere.amount);

    let mut p = Pyranose::new(sphere.amount);

    let vec_of_pyranoses = reconstruct_ring::reconstruct_coordinates(
        &projection,
        sphere.amount,
        local_elevation::cremerpople_evelation(&sphere),
    );

    for (i, pyr) in vec_of_pyranoses.iter().enumerate() {
        p.alpha1[i] = dihedral(pyr.p5, pyr.p1, pyr.p3, pyr.p2);
        p.alpha2[i] = dihedral(pyr.p1, pyr.p3, pyr.p5, pyr.p4);
        p.alpha3[i] = dihedral(pyr.p3, pyr.p5, pyr.p1, pyr.p6);
    }

    // Dihedral function has values ORCA-ready
    p
}
