pub fn bubble_sort(vec: Vec<i32>) -> Vec<i32> {
    let mut sorted: Vec<i32> = vec;
    for i in 0..sorted.len() {
        for j in 0..sorted.len() - 1 - i {
            if sorted[j] > sorted[j + 1] {
                println!("swapping {} with {}", sorted[j], sorted[j + 1]);
                let tmp = sorted[j];
                sorted[j] = sorted[j + 1];
                sorted[j + 1] = tmp;
            }
        }
    }
    sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let v = vec![1, 8, 22, 56, -11, -5, 0, -99, 99, 75];
        let sorted = vec![-99, -11, -5, 0, 1, 8, 22, 56, 75, 99];
        assert_eq!(sorted, bubble_sort(v));
    }
}
