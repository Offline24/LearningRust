pub fn let_is_immutable() {
    let x = 4;
    println!("immutable x = {}", x);
    // x = 3; // produce compile error
}

pub fn let_with_mut_is_mutable() {
    let mut x = 4;
    println!("mutable x = {}", x);
    x = 3;
    println!("mutable x = {}", x);
}

pub fn mutable_arguments() {
    println!("start values");
    let mut mut_x = 10;
    let y = 10;
    println!("mut_x = {}\ty = {}", mut_x, y);

    println!("mutable_argument");
    mutable_argument(mut_x);
    mutable_argument(y);
    println!("mut_x = {}\ty = {}", mut_x, y);

    println!("after mutable_reference_argument");
    mutable_reference_argument(&mut mut_x);
    println!("mut_x = {}", mut_x);

    println!("after mutable_reference_argument_with_shadowing");
    mutable_reference_argument_with_shadowing(&mut mut_x);
    println!("mut_x = {}", mut_x);
}

const CONSTANT_VALUE: i32 = 100_000;

#[allow(dead_code)]
fn get_constant_value() -> i32 {
    CONSTANT_VALUE
} 

#[allow(unused_assignments)]
#[allow(unused_variables)]
fn mutable_argument(mut x: i32) {
    x = 0;
}

fn mutable_reference_argument(x: &mut i32) {
    *x = 0;
}

fn mutable_reference_argument_with_shadowing(x: &mut i32) {
    println!("\targument: {}", x);
    *x = 1;
    println!("\t*x = 1: {}", x);
    let x = 2;
    println!("\tlet x = 2: {}", x);
    let mut x = 3;
    println!("\tlet mut x = 3: {}", x);
    x = 4;
    println!("\tx = 4: {}", x);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutable_argument_do_not_change_on_caller_side() {
        let x = 10;
        mutable_argument(x);
        assert_eq!(x, 10);
    }

    #[test]
    #[allow(unused_mut)]
    fn mutable_argument_do_not_change_on_caller_side2() {
        let mut x = 10;
        mutable_argument(x);
        assert_eq!(x, 10);
    }

    #[test]
    fn get_constant_value_test() {
        assert_eq!(get_constant_value(), CONSTANT_VALUE);
    }
}
