pub fn search_by_sqrt(breaks: Vec<bool>) -> Option<usize> {
    let offset = f32::sqrt(breaks.len() as f32 - 0.5) as usize;
    let mut i = offset;
    while i < breaks.len() {
        if breaks[i] {
            break;
        }
        i += offset;
    }
    i -= offset;
    let mut j = 0;
    while j <= offset {
        if i >= breaks.len() {
            return None;
        }
        if breaks[i] {
            println!("found it at {}, breaks: {}", i, breaks[i]);
            return Some(i);
        }
        j += 1;
        i += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breaks1() {
        let breaks1 = vec![true, true, true, true, true, true, true, true, true, true];
        assert_eq!(search_by_sqrt(breaks1), Some(0));
    }

    #[test]
    fn test_breaks2() {
        let breaks2 = vec![
            false, false, false, false, false, false, true, true, true, true,
        ];
        assert_eq!(search_by_sqrt(breaks2), Some(6));
    }

    #[test]
    fn test_breaks3() {
        let breaks3 = vec![
            false, false, false, false, false, false, false, false, false, true,
        ];
        assert_eq!(search_by_sqrt(breaks3), Some(9));
    }

    #[test]
    fn test_breaks4() {
        let breaks4 = vec![false, true, true, true, true, true, true, true, true, true];
        assert_eq!(search_by_sqrt(breaks4), Some(1));
    }

    #[test]
    fn test_breaks5() {
        let breaks5 = vec![
            false, false, false, false, false, false, false, false, false, false,
        ];
        assert_eq!(search_by_sqrt(breaks5), None);
    }
}
