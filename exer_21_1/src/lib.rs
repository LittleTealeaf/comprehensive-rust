#[allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    let string = cc_number.replace(" ", "");
    if string.len() < 2 {
        return false;
    }
    if let Err(_) = string.parse() as Result<u64, _> {
        return false;
    }
    let mut nums: Vec<u8> = string
        .chars()
        .into_iter()
        .map(|c| -> u8 { String::from(c).parse().unwrap() })
        .collect();

    let double_on = if nums.len() % 2 == 0 {0} else {1};

    for i in 0..nums.len() {
        if i % 2 == double_on  {
            nums[i] *= 2;
            if nums[i] >= 10 {
                nums[i] -= 9;
            }
        }
    }

    u8::to_string(&(nums.iter().sum())).chars().last().unwrap() == '0'
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("   "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}
