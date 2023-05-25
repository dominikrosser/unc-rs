pub mod uncertain_float;
pub mod und;
mod tests;

use uncertain_float::Uf64;

fn main() {
    add_sub_mul_div_example();
    finite_difference_approx_apply_example();
    central_difference_approx_with_order4_example();
    ndarray_print_example();
    ndarray_print_example();
    ndarray_add_example();
    ndarray_sin_example();
}

fn add_sub_mul_div_example() {
    println!("add_sub_mul_div_example");
    let a = Uf64::new(1.0, 0.1);
    let b = Uf64::new(2.0, 0.2);
     
    println!("a + b = {:?}", a + b);
    println!("a - b = {:?}", a - b);
    println!("a * b = {:?}", a * b);
    println!("a / b = {:?}", a / b);
}

fn finite_difference_approx_apply_example() {
    println!("finite_difference_approx_apply_example");
    let x = Uf64::new(3.14, 0.01);
    let sin_x = x.apply_with_finite_difference_approx(&f64::sin);
    println!("sin({:?}) = {:?}", x, sin_x);

    let x = Uf64::new(1.001, 0.00001);
    let ln_x = x.apply_with_finite_difference_approx(&f64::ln);
    println!("ln({:?}) = {:?}", x, ln_x);
}

fn central_difference_approx_with_order4_example() {
    println!("central_difference_approx_with_order4_example");
    let x = Uf64::new(3.14, 0.01);
    let sin_x = x.apply_with_central_difference_approx_order_four(&f64::sin);
    println!("sin({:?}) = {:?}", x, sin_x);

    let x = Uf64::new(1.001, 0.00001);
    let ln_x = x.apply_with_central_difference_approx_order_four(&f64::ln);
    println!("ln({:?}) = {:?}", x, ln_x);
    
    let tmp = ln_x.apply(&f64::exp);
    println!("exp(ln({:?})) = {:?}", x, tmp);
}

fn apply_example() {
    println!("apply_example");
    let x = Uf64::new(1.001, 0.00001);
    let ln_x = x.apply_with_central_difference_approx_order_four(&f64::ln);
    let tmp = ln_x.apply(&f64::exp);
    println!("exp(ln({:?})) = {:?}", x, tmp);
}

fn ndarray_print_example() {
    println!("ndarray_print_example");
    let arr = ndarray::arr1(&[
        Uf64::new(1.0, 0.1),
        Uf64::new(2.0, 0.2),
        Uf64::new(3.0, 0.3),
        Uf64::new(4.0, 0.4),
    ]);
    println!("arr = {:?}", arr);
}

fn ndarray_add_example() {
    println!("ndarray_add_example");
    let arr1 = ndarray::arr1(&[
        Uf64::new(1.0, 0.1),
        Uf64::new(2.0, 0.2),
        Uf64::new(3.0, 0.3),
        Uf64::new(4.0, 0.4),
    ]);
    let arr2 = ndarray::arr1(&[
        Uf64::new(1.0, 0.1),
        Uf64::new(2.0, 0.2),
        Uf64::new(3.0, 0.3),
        Uf64::new(4.0, 0.4),
    ]);
    let sum = arr1 + arr2;
    println!("sum = {:?}", sum);
}

fn ndarray_sin_example() {
    let arr = ndarray::arr1(&[
        Uf64::new(1.0, 0.1),
        Uf64::new(2.0, 0.2),
        Uf64::new(3.0, 0.3),
        Uf64::new(4.0, 0.4),
    ]);
    let sin_arr = und::apply(&arr, &f64::sin);
    println!("sin_arr = {:?}", sin_arr);
}