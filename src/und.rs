/// ndarray utils

use ndarray::{Array, Array1, Array2, Array3, Array4, Array5, Array6, ArrayView1, ArrayView2, ArrayView3, ArrayView4, ArrayView5, ArrayView6, Axis, Ix1, Ix2, Ix3, Ix4, Ix5, Ix6};
use ndarray::Dimension;
use crate::uncertain_float::Uf64;
use crate::uncertain_float::RealFunc;

/// apply function to each element of ndarray with the UncertainFloat type
pub fn apply<D> (
    arr: &Array<Uf64, D>,
    f: &RealFunc,
) -> Array1<Uf64>
where D: Dimension {
    let mut result = Array1::<Uf64>::zeros(arr.len());
    for (i, x) in arr.iter().enumerate() {
        result[i] = x.apply(f);
    }
    result
}
