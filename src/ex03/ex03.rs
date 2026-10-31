#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    fn to_bool(s: &str) -> bool {
        todo!();
    }

    #[test]
    fn ex03_basic_tests() {
        assert!(!to_bool("0"));
        assert!(to_bool("1"));
        assert!(!to_bool("1!"));
        assert!(to_bool("0!"));
        assert!(to_bool("11&"));
        assert!(!to_bool("10&"));
        assert!(to_bool("10|"));
        assert!(to_bool("01|"));
        assert!(!to_bool("00|"));
        assert!(to_bool("10^"));
        assert!(!to_bool("11^"));
        assert!(!to_bool("10>"));
        assert!(to_bool("01>"));
        assert!(!to_bool("10="));
        assert!(to_bool("11="));
    }

    #[test]
    fn ex03_subject_tests() {
        assert!(!to_bool("10&"));
        assert!(to_bool("10|"));
        assert!(to_bool("11>"));
        assert!(!to_bool("10="));
        assert!(to_bool("1011||="));
    }

    #[test]
    fn ex03_advanced_tests() {
        assert!(to_bool("1011||="));
        assert!(to_bool("111&!!!1|01=|=11>^0|0!1^11>1|0>1^>10^1|>10^>^"));
    }
}
