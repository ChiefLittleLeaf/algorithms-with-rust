pub fn linear_search(arr: Vec<u8>, search: u8) -> Result<u8, String> {
    for i in arr.iter() {
        if i == &search {
            return Ok(*i);
        }
    }
    Err(String::from("result not in array"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search() {
        let arr: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(linear_search(arr.clone(), 1), Ok(1));
        assert_eq!(linear_search(arr.clone(), 9), Ok(9));
        assert_ne!(linear_search(arr.clone(), 6), Ok(5));
        assert_eq!(linear_search(arr.clone(), 4), Ok(4));
        assert_eq!(
            linear_search(arr.clone(), 25),
            Err(String::from("result not in array"))
        );
    }
}
