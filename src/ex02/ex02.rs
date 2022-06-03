fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}

fn main() {
    for n in 0..127 {
        let result = gray_code(n);
        println!("{:3} => {:3} ({:07b})", n, result, result);
    }
}

#[test]
fn test_gray_code() {
    assert_eq!(gray_code(0), 0);
    assert_eq!(gray_code(1), 1);
    assert_eq!(gray_code(2), 3);
    assert_eq!(gray_code(3), 2);
    assert_eq!(gray_code(4), 6);
    assert_eq!(gray_code(5), 7);
    assert_eq!(gray_code(6), 5);
    assert_eq!(gray_code(7), 4);
    assert_eq!(gray_code(8), 12);
}
