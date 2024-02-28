pub fn substract_10(num: u32) -> u32 {
    num - 10
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn substract_10s_test(){
        let x: u32 = 100;
        let y: u32 = substract_10(x);

        println!("x and y are from tset: {} {}", x, y);
        assert_eq!(y, 90);
    }
}