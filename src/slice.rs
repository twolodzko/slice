#[derive(Debug, PartialEq)]
pub struct SliceSpec {
    pub filename: String,
    pub start: Option<usize>,
    pub end: Option<usize>,
}

impl From<&str> for SliceSpec {
    fn from(s: &str) -> Self {
        let (filename, start, end) = split_path(s);
        let mut slice = Self {
            filename,
            start: None,
            end: None,
        };

        if let Some(s) = start {
            if s != "" {
                match s.parse::<usize>() {
                    Ok(n) => slice.start = Some(n),
                    Err(_) => slice.filename = format!("{}:{}", slice.filename, s),
                }
            }
        };

        if let Some(s) = end {
            if s != "" {
                match s.parse::<usize>() {
                    Ok(n) => slice.end = Some(n),
                    Err(_) => slice.filename = format!("{}:{}", slice.filename, s),
                }
            }
        };

        slice
    }
}

#[inline]
fn split_path(path: &str) -> (String, Option<&str>, Option<&str>) {
    match &path.split(":").collect::<Vec<_>>()[..] {
        [] => ("".to_string(), None, None),
        [file] => (file.to_string(), None, None),
        [file, start] => (file.to_string(), Some(start), None),
        [file @ .., start, end] => (file.join(":"), Some(start), Some(end)),
    }
}

#[cfg(test)]
mod tests {
    use super::SliceSpec;
    use test_case::test_case;

    #[test_case("", "", None, None; "empty")]
    #[test_case("::", "", None, None; "empty with empty bounds")]
    #[test_case(":1:5", "", Some(1), Some(5); "empty with bounds")]
    #[test_case(":::", ":", None, None; "empty with colon")]
    #[test_case("::::", "::", None, None; "empty with two colons")]
    #[test_case("foo", "foo", None, None; "name only")]
    #[test_case("foo::", "foo", None, None; "empty slice bounds")]
    #[test_case("foo:1:", "foo", Some(1), None; "lower slice bound")]
    #[test_case("foo::5", "foo", None, Some(5); "upper slice bound")]
    #[test_case("foo:2:6", "foo", Some(2), Some(6); "both slice bounds")]
    #[test_case(
        "~/Strange:path-name:345:5/Directory Name/file.txt:567:1078",
        "~/Strange:path-name:345:5/Directory Name/file.txt", Some(567), Some(1078)
        ; "long and messy")]
    fn from(path: &str, filename: &str, start: Option<usize>, end: Option<usize>) {
        let result = SliceSpec::from(path);
        let expected = SliceSpec {
            filename: filename.to_string(),
            start,
            end,
        };
        assert_eq!(result, expected)
    }
}
