use puckers::arguments::Flags;
use puckers::sixring::sixring;
use puckers::torsion_typing::TorsionType;

use assert_float_eq::*;

#[test]
fn test_sixring_axes_generation() {
    let flag = Flags {
        torsion_type: Some(TorsionType::Sixring),
        num: 630,
    };

    let sixring_axes = sixring(&flag);

    assert_float_absolute_eq!(sixring_axes.alpha1[0], 139.881, 0.001);
    assert_float_absolute_eq!(sixring_axes.alpha2[0], 146.537, 0.001);
    assert_float_absolute_eq!(sixring_axes.alpha3[0], 139.881, 0.001);

    assert_float_absolute_eq!(sixring_axes.alpha1[369], 134.750, 0.001);
    assert_float_absolute_eq!(sixring_axes.alpha2[369], -149.389, 0.001);
    assert_float_absolute_eq!(sixring_axes.alpha3[369], -140.362, 0.001);
}
