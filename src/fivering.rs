use ndarray::Array1;

use crate::arguments::Flags;
use crate::Torsions;




#[allow(dead_code)]
pub fn fivering(flags : &Flags) -> Torsions {
    
    let a1 : Array1<f64> = Array1::linspace(-60., 60., flags.num as usize);
    let a2 = a1.clone();

    let t = generate_nu1_and_nu3(&flags.num, a1, a2);

    t
}


#[allow(dead_code)]
fn generate_nu1_and_nu3(num : &u32, array1: Array1<f64>, array2: Array1<f64>) -> Torsions {
    
    let _sizeof : u32 = num ** num;
    let t = Torsions {
        array1 : Array1::<f64>::zeros(_sizeof as usize),
        array2 : Array1::<f64>::zeros(_sizeof as usize),
    };

    t //return Torsions struct
}
