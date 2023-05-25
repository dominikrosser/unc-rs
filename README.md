# unc-rs
Inspired by the python package uncertainties.
Already compatible with Add, Sub, Mul, Div, and generally functions of the form ´f: Fn(f64) -> f64´. With these functions a stupid approximation of the derivative is currently used. Can be used in ´ndarray´, and functions on the array can be called using the ´und´ mod. No correlations are involved in the calculations, just stupid derivative approximations. It is to be converted to a library and further tests are needed (i banged the current version of this project out in a few hours), but results (for continuous functions like sine, etc. and values with not too low in magnitude) of this code have been pretty close to the uncertainties package.

## Examples
See main.rs

## TODO
- improve derivative approx.
- correlations when calculations are done with ndarray
- make lib, (into public crate when tested and stable.....)
- ...