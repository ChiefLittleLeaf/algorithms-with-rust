pub fn binary_search(arr: Vec<usize>, search: usize) -> bool {
    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let m = low + (high - low) / 2;
        let v = arr[m];
        if v == search {
            return true;
        }
        if v > search {
            high = m;
        } else {
            low = m + 1;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let arr: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert!(binary_search(arr.clone(), 3));
        assert!(binary_search(arr.clone(), 8));
        assert!(!binary_search(arr.clone(), 10));
        assert!(!binary_search(arr.clone(), 25));
    }
}
