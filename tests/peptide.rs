use puckers::arguments::Flags;
use puckers::peptide::peptide;
use puckers::torsion_typing::TorsionType;

use assert_float_eq::*;

#[test]
fn test_peptide_axes_generation() {
    let flag = Flags {
        torsion_type: Some(TorsionType::Peptide),
        num: 37,
    };

    let peptide_axes = peptide(&flag);

    assert_float_absolute_eq!(peptide_axes.phi[0], 0.000, 0.0001);
    assert_float_absolute_eq!(peptide_axes.psi[8], 80.000, 0.0001);

    assert_float_absolute_eq!(peptide_axes.phi[100], 20.000, 0.0001);
    assert_float_absolute_eq!(peptide_axes.psi[100], 260.000, 0.0001);
}
