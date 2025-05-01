use ndarray::Array2;
use std::f64::consts::PI;

use crate::sixring::geometry::{subtract_arr, Coordinate, LinAlg, RotMatrix, RotationMatrix};
use crate::sixring::ring_partition::ProjectionPartition;

/// Since we work it large array sizes, depending on the query sizes,
/// We will have to work with more manageable data-wise and working on such large array sizes will
/// make the code look more complicated, with more indented loops and indexing.
///
/// That is why, with the information of the ring partition struct, we will rebuild every spherical
/// coordinate separately in a function
/// Essentially, we will design a struct that will hold several method to make code more legible.
/// Then, reconstruct_coordinates() returns a Vec<SixRingAtoms>
///

// fields s11, s25 and s31 are never read. Included for declarative purposes
#[derive(Debug)]
struct PointPositions {
    //s11: Coordinate,
    s12: Coordinate,
    s13: Coordinate,
    s23: Coordinate,
    s24: Coordinate,
    //s25: Coordinate,
    s35: Coordinate,
    s36: Coordinate,
    //s31: Coordinate,
}

#[derive(Debug)]
pub struct SixRingAtoms {
    pub p1: Coordinate,
    pub p2: Coordinate,
    pub p3: Coordinate,
    pub p4: Coordinate,
    pub p5: Coordinate,
    pub p6: Coordinate,
}

impl SixRingAtoms {
    /// In the C version of this code, the guy iterates over the three coordinate points
    /// while only the first two are non-zero.
    /// Check out later if this has an impact, or else we change it to only iterate over that
    /// and leave the third value in the array as 0.
    fn calculate_geometric_center(&self) -> Coordinate {
        [0, 1, 2].map(|i| {
            (self.p1[i] + self.p2[i] + self.p3[i] + self.p4[i] + self.p5[i] + self.p6[i]) / 6.
        })
    }
}

/// Return a Vec of all the conformers' atoms' position in cartesian coordinates
/// Then we will derive all the alpha angles (the improper dihedrals) in a next function
pub fn reconstruct_coordinates(
    proj: &ProjectionPartition,
    sphere_size: usize,
    z_j: Array2<f64>,
) -> Vec<SixRingAtoms> {
    // proj : projections and partitioning.

    let mut pyranosecoordinates = Vec::with_capacity(sphere_size);

    for i in 0..sphere_size {
        // Add the local evelation already as the z-coordinate to the final molecule's array
        let mut sixring = SixRingAtoms {
            p1: [0., 0., z_j[[i, 0]]],
            p2: [0., 0., z_j[[i, 1]]],
            p3: [0., 0., z_j[[i, 2]]],
            p4: [0., 0., z_j[[i, 3]]],
            p5: [0., 0., z_j[[i, 4]]],
            p6: [0., 0., z_j[[i, 5]]],
        };

        //	S11[0] = 0.;		S11[1] = 0.;
        //	S12[0] = -rpij[0];	S12[1] = 0.;
        //	S13[0] = -rpij[0]+rpij[1]*COSbetapijk[0];	S13[1] = rpij[1]*SINbetapijk[0];
        //
        //	S23[0] = OQ+rpij[3]-rpij[2]*COSbetapijk[2];	S23[1] = rpij[2]*SINbetapijk[2];
        //	S24[0] = OQ+rpij[3];	S24[1] = 0.;
        //	S25[0] = OQ;		S25[1] = 0.;
        //
        //	S35[0] = rpij[5]-rpij[4]*COSbetapijk[4];	S35[1] = rpij[4]*SINbetapijk[4];
        //	S36[0] = rpij[5];	S36[1] = 0.;
        //	S31[0] = 0.;	S31[1] = 0.;
        //
        //
        let pyranose = PointPositions {
            //s11 : [0., 0., 0.],
            s12: [-proj.rpij[[i, 0]], 0., 0.],
            s13: [
                (-proj.rpij[[i, 0]]) + (proj.rpij[[i, 1]] * proj.cosbpijk[[i, 0]]),
                proj.rpij[[i, 1]] * proj.sinbpijk[[i, 0]],
                0.,
            ],
            s23: [
                (proj.oq[i] + proj.rpij[[i, 3]]) - (proj.rpij[[i, 2]] * proj.cosbpijk[[i, 2]]),
                proj.rpij[[i, 2]] * proj.sinbpijk[[i, 2]],
                0.,
            ],
            s24: [proj.oq[i] + proj.rpij[[i, 3]], 0., 0.],
            //s25 : [proj.oq[i] , 0., 0.],
            s35: [
                proj.rpij[[i, 5]] - (proj.rpij[[i, 4]] * proj.cosbpijk[[i, 4]]),
                proj.rpij[[i, 4]] * proj.sinbpijk[[i, 4]],
                0.,
            ],
            s36: [proj.rpij[[i, 5]], 0., 0.],
            //s31 : [0., 0., 0.],
        };

        //	rho1 = atan2(S13[1],S13[0]);
        //	rho2 = atan2(S23[1],S23[0]-OQ);
        //	rho3 = atan2(S35[1],S35[0]);
        let rho1 = pyranose.s13[1].atan2(pyranose.s13[0]);
        let rho2 = pyranose.s23[1].atan2(pyranose.s23[0] - proj.oq[i]);
        let rho3 = pyranose.s35[1].atan2(pyranose.s35[0]);

        //	pO[0] = 0.;	pO[1] = 0.;
        //	pP[0] = (OP*OP+OQ*OQ-QP*QP)/(2.*OQ);
        //	pP[1] = sqrt( OP*OP - ( (OP*OP+OQ*OQ-QP*QP)*(OP*OP+OQ*OQ-QP*QP) )/(4.*OP*OP) );
        //	pQ[0] = OQ;	pQ[1] = 0.;
        //	pO[2]=pP[2]=pQ[2]=0.;
        //	rhoPS1 = atan2(pP[1],pP[0]);
        //	rhoPS2 = atan2(pP[1],pP[0]-OQ);
        //	sigma1 = rho1 - rhoPS1 ;
        //	sigma2 = rhoPS2 - rho2;
        //	sigma3 = rho3;
        let p_o: Coordinate = [0., 0., 0.]; //pO
        let p_p: Coordinate = [
            (proj.op[i].powi(2) + proj.oq[i].powi(2) - proj.qp[i].powi(2)) / (2. * proj.oq[i]),
            (proj.op[i].powi(2)
                - (((proj.op[i].powi(2) + proj.oq[i].powi(2) - proj.qp[i].powi(2)).powi(2))
                    / (4. * proj.op[i].powi(2))))
            .sqrt(),
            0.,
        ]; //pP
        let p_q: Coordinate = [proj.oq[i], 0., 0.]; //pQ

        // rhoPS1 = atan2(pP[1],pP[0]);
        // rhoPS2 = atan2(pP[1],pP[0]-OQ);
        let rho_ps1 = p_p[1].atan2(p_p[0]);
        let rho_ps2 = p_p[1].atan2(p_p[0] - proj.oq[i]);

        let sigma1 = rho1 - rho_ps1;
        let sigma2 = rho_ps2 - rho2;
        let sigma3 = rho3;

        // p1, p3, p5 already exist on the xy'-plane, so need only to rotate p2,p4,p6
        let tmp_sixring = SixRingAtoms {
            p1: p_o,
            p2: RotationMatrix::new(-sigma1).apply_rotation(pyranose.s12),
            p3: p_p,
            p4: RotationMatrix::new(sigma2)
                .apply_rotation(subtract_arr(pyranose.s24, p_q))
                .add_arr(&p_q),
            p5: p_q,
            p6: RotationMatrix::new(-sigma3).apply_rotation(pyranose.s36),
        };

        // Calculate geometric center
        let p_g: Coordinate = tmp_sixring.calculate_geometric_center();
        // Derive final rotation matrix
        let rho_g = (PI / 2.) + p_g[1].atan2(p_g[0]);
        let rot4 = RotationMatrix::new(-rho_g);

        // final rotation
        sixring.p1[0] = rot4.apply_rotation_around_g(subtract_arr(tmp_sixring.p1, p_g), 0);
        sixring.p2[0] = rot4.apply_rotation_around_g(subtract_arr(tmp_sixring.p2, p_g), 0);
        sixring.p3[0] = rot4.apply_rotation_around_g(subtract_arr(tmp_sixring.p3, p_g), 0);
        sixring.p4[0] = rot4.apply_rotation_around_g(subtract_arr(tmp_sixring.p4, p_g), 0);
        sixring.p5[0] = rot4.apply_rotation_around_g(subtract_arr(tmp_sixring.p5, p_g), 0);
        sixring.p6[0] = rot4.apply_rotation_around_g(subtract_arr(tmp_sixring.p6, p_g), 0);

        sixring.p1[1] = rot4.apply_rotation_around_g(subtract_arr(tmp_sixring.p1, p_g), 1);
        sixring.p2[1] = rot4.apply_rotation_around_g(subtract_arr(tmp_sixring.p2, p_g), 1);
        sixring.p3[1] = rot4.apply_rotation_around_g(subtract_arr(tmp_sixring.p3, p_g), 1);
        sixring.p4[1] = rot4.apply_rotation_around_g(subtract_arr(tmp_sixring.p4, p_g), 1);
        sixring.p5[1] = rot4.apply_rotation_around_g(subtract_arr(tmp_sixring.p5, p_g), 1);
        sixring.p6[1] = rot4.apply_rotation_around_g(subtract_arr(tmp_sixring.p6, p_g), 1);

        pyranosecoordinates.push(sixring);
    }

    pyranosecoordinates // return Vec<SixRingAtoms>
}

#[cfg(test)]
mod reconstruction {
    use super::*;

    #[test]
    pub fn test_indexing() {
        let sr = SixRingAtoms {
            p1: [1.16, 1.23, 0.45],
            p2: [0.02, 2.97, 0.34],
            p3: [3.24, 3.69, 2.81],
            p4: [1.21, 0.71, 3.74],
            p5: [2.95, 0.11, 0.28],
            p6: [0.87, 2.99, 0.25],
        };

        let mut p1: Coordinate = [0., 0., 0.]; // the geometric center

        for i in 0..3 {
            p1[i] = (sr.p1[i] + sr.p2[i] + sr.p3[i] + sr.p4[i] + sr.p5[i] + sr.p6[i]) / 6.
        }

        let p2 = [0, 1, 2]
            .map(|i| (sr.p1[i] + sr.p2[i] + sr.p3[i] + sr.p4[i] + sr.p5[i] + sr.p6[i]) / 6.);

        // if the arrays are equal, their iterated sum should also be equal
        assert_eq!(p1.iter().sum::<f64>(), p2.iter().sum());
    }
}
