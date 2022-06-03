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

fn multiplier(a: u32, b: u32) -> u32 {
    let mut result = 0;
    let mut multiplicand = a;
    let mut multiplier = b;

    while multiplier != 0 {
        if multiplier & 1 == 1 {
            result = adder(result, multiplicand);
        }
        multiplier = multiplier >> 1;
        multiplicand = multiplicand << 1;
    }
    result
}

fn main() {
    let a = 6;
    let b = 7;
    println!("{} * {} = {}", a, b, multiplier(a, b));
}

#[test]
fn test_multiplier() {
    fn test(a: u32, b: u32) {
        assert_eq!(multiplier(a, b), a.wrapping_mul(b));
    }
    let max = std::u32::MAX;
    test(27, 15);
    test(123, 456);
    test(0, 0);
    test(0, 1);
    test(1, 0);
    test(1, 1);
    test(1, 2);
    test(2, 2);
    test(2, 4);
    test(4, 2);
    test(max, 2);
    test(max, 3);
    test(max, 4);
    test(max, max);
    test(max, max - 1);
}
