// src/lib.rs

pub fn multiply_array(slice: &[i32]) -> i32 {
    let mut product = 1;
    for &val in slice  {
        product *= val;
    }
    product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_array() {
        let arr = [2, 3, 4];
        let product = multiply_array(&arr);
        assert_eq!(product, 24);
    }
}

fn main() {
    println!("Hello, world!");
}
