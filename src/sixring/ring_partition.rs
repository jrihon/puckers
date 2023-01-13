//use ndarray::prelude::*;
use ndarray::{ArrayBase, Ix2, DataOwned};

// Make a trait where we can implement our own function on the ArrayBase<S,D> struct.
// Technically, we do not need a trait for this, but what the heck, nothing can stop me.
pub trait RingPartition {
    fn ring_partition(&self) -> PartsOfRing;
    
}


// Store ring partitioning in the struct
pub struct PartsOfRing {
    qp : i32

}




impl<S, A> RingPartition for ArrayBase<S, Ix2>
where 
    S : DataOwned<Elem = A>,
{
    fn ring_partition(&self) -> PartsOfRing {
        println!("Ring Partition segment");


        PartsOfRing { qp: 3 }

    }
}
