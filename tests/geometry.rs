use assert_float_eq::*;
use puckers::sixring::geometry;

#[test]
pub fn planarity() {
    //N1   6.105   8.289   4.633
    //C2   7.360   7.768   4.827
    //N3   7.390   6.603   5.551
    //C4   6.301   5.942   6.079

    let planar = geometry::dihedral(
        [6.105, 8.289, 4.633],
        [7.360, 7.768, 4.827],
        [7.390, 6.603, 5.551],
        [6.301, 5.942, 6.079],
    );
    // assert up until the 3rd decimal
    assert_float_absolute_eq!(planar, -0.07, 0.001)
}

#[test]
pub fn chi_angle() {
    //O4'   5.157  10.381   4.681
    //C1'   5.981   9.551   3.863
    //N1    6.105   8.289   4.633
    //C2    7.360   7.768   4.827

    let chi = geometry::dihedral(
        [5.157, 10.381, 4.681],
        [5.981, 9.551, 3.863],
        [6.105, 8.289, 4.633],
        [7.360, 7.768, 4.827],
    );
    // assert up until the 3rd decimal
    assert_float_absolute_eq!(chi, -130.214, 0.001)
}

#[test]
pub fn subtract_points() {
    let a = [1., 2., 3.];
    let b = [4., 5., 6.];

    let c = geometry::subtract_arr(b, a);
    assert_float_absolute_eq!(
        c.iter().sum::<f64>(),
        [3., 3., 3.,].iter().sum::<f64>(),
        0.001
    )
}
