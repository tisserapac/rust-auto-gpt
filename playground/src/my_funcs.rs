
/// Function: add_five
/// 
/// # Arguments (num: u32)
/// 
/// # Returns u32
/// 
/// # Example
/// ```
/// let x = 5;
/// let y = add_five(x);
/// ```
/// 
/**
 * This is a multiline block.
 * This is for the function add_five.
 */
pub fn add_five(num: u32) -> u32 {
    num + 5 // add 5
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_five_test(){
        let x: u32 = 100;
        let y: u32 = add_five(x);

        println!("x and y are from tset: {} {}", x, y);
        assert_eq!(y, 105);
    }
}

