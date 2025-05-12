use puckers::arguments::Flags;
use puckers::fivering::fivering;
use puckers::torsion_typing::TorsionType;

use assert_float_eq::*;

#[test]
fn test_fivering_axes_generation() {
    let flag = Flags {
        torsion_type: Some(TorsionType::Fivering),
        num: 21,
    };

    let fivering_axes = fivering(&flag);

    assert_float_absolute_eq!(fivering_axes.nu1[0], 13.274, 0.001);
    assert_float_absolute_eq!(fivering_axes.nu3[0], 83.808, 0.001);

    assert_float_absolute_eq!(fivering_axes.nu1[100], 50.285, 0.001);
    assert_float_absolute_eq!(fivering_axes.nu3[100], 7.964, 0.001);

    assert_float_absolute_eq!(fivering_axes.nu1[199], 4.854, 0.001);
    assert_float_absolute_eq!(fivering_axes.nu3[199], 4.854, 0.001);
}
