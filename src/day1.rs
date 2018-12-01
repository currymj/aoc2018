pub fn compute_final_frequency(freqs: &Vec<i32>) -> i32 {
    freqs.iter().fold(0, |sum, x| {sum + x})
}

pub fn compute_repeat_frequency(freqs: &Vec<i32>) -> i32 {
    unimplemented!()
}

#[cfg(test)]
mod tests{
    use super::compute_final_frequency;
    #[test]
    fn test1() {
        let input1 = [1, -2, 3, 1].to_vec();
        let result = compute_final_frequency(&input1);
        assert_eq!(result, 3)
    }
    #[test]
    fn test2() {
        let input1 = [1,1,1].to_vec();
        let result = compute_final_frequency(&input1);
        assert_eq!(result, 3)
    }
    #[test]
    fn test3() {
        let input1 = [1,1,-2].to_vec();
        let result = compute_final_frequency(&input1);
        assert_eq!(result, 0)
    }
    #[test]
    fn test4() {
        let input1 = [-1, -2, -3].to_vec();
        let result = compute_final_frequency(&input1);
        assert_eq!(result, -6)
    }
    #[test]
    fn test5() {
        let input1 = [1, -1].to_vec();
        let result = compute_repeat_frequency(&input1);
        assert_eq!(result, 0)
    }
    #[test]
    fn test6() {
        let input1 = [3, 3, 4, -2, -4].to_vec();
        let result = compute_repeat_frequency(&input1);
        assert_eq!(result, 10)
    }
    #[test]
    fn test7() {
        let input1 = [6, 3, 8, 5, -6].to_vec();
        let result = compute_repeat_frequency(&input1);
        assert_eq!(result, 5)
    }
    #[test]
    fn test8() {
        let input1 = [7, 7, -2, -7, -4].to_vec();
        let result = compute_repeat_frequency(&input1);
        assert_eq!(result, 14)
    }
}