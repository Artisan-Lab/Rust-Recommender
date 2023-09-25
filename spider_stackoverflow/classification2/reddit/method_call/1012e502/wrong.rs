use ndarray::prelude::*;
use ndarray::{Array, ArrayBase, OwnedRepr, Array1, Array2};

fn main() {
    // All rows in same time
    let mut spec = array![[1, 2, 3, 4], [5,6,7,8]]; // mxn
    let exponencial = array![[1,1,1,1]]; // matrix 1xn

    
    let result = spec.assign(&(&spec*&exponencial));
	// or spec = &spec*&exponencial;
	
// 	In the form of a loop. one row each iteration.
// This is the implementation that I want to make to be able 
// to implement it to be able to compare with other languages
    // 1)
    let mut spec = array![[1, 2, 3, 4], [5,6,7,8]]; // mxn
    let exponencial = array![[1,1,1,1]]; // matrix 1xn
    spec.slice_mut(s![0,..]).assign(&(&(spec.row(0))*&exponencial.row(0)));
}
