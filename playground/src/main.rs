mod my_funcs;
mod other_funcs;

use crate::my_funcs::add_five;
use crate::other_funcs::minus_funcs::substract_10;

fn main() {
    let mut x: u32 = 54;
    println!("x is {}", x);

    let y: u32 = add_five(x);
    println!("y is {}", y);

    x = 70;
    let z: u32 = substract_10(x);
    println!("z is {}", z);

}
