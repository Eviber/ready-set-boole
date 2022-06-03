fn adder(a: u32, b: u32) -> u32 {
    let mut sum = a ^ b;
    let mut carry = (a & b) << 1;
    while carry != 0 {
        let tmp = sum;
        sum = carry ^ tmp;
        carry = (carry & tmp) << 1;
    }
    sum
}

fn main() {
    let a = 27;
    let b = 15;
    println!("{} + {} = {}", a, b, adder(a, b));
}

#[test]
fn test_adder() {
    assert_eq!(3, adder(1, 2));
    assert_eq!(5, adder(2, 3));
    assert_eq!(10, adder(4, 6));
    assert_eq!(20, adder(8, 12));
    assert_eq!(30, adder(15, 15));
    assert_eq!(0, adder(0, 0));
    assert_eq!(1, adder(0, 1));
    assert_eq!(2, adder(1, 1));
    assert_eq!(999, adder(500, 499));
    assert_eq!(0, adder(1, u32::MAX));
}
