pub fn sum(numbers: Vec<String>) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    fn into_strings<T>(input: T) -> Vec<String>
    where
        T: IntoIterator<Item = i64>,
    {
        input.into_iter().map(|n| n.to_string()).collect()
    }

    #[test]
    fn it_works() {
        assert_eq!(sum(into_strings([0, 1, 2, 3])), 6);
        assert_eq!(sum(into_strings([-27, -0, 7])), -20);
        assert_eq!(sum(into_strings([])), 0);
        let mut test_case_4: Vec<String> = into_strings([3, 5, 7]);
        test_case_4.append(&mut vec![String::from("handle_me")]);
        assert_eq!(sum(test_case_4), 15);
    }
}
