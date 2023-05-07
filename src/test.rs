#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_csv_file() {
        let expected = Ok(vec![(1, 2), (2, 3), (3, 4)]);
        let actual = read_csv_file("test.csv");
        assert_eq!(expected, actual);
    }
}
