fn i32_to_bcd(x: i32) -> i32 {
    let mut power_of_ten: i32 = 10;
    let mut digits: i32 = 2;
    while (x / power_of_ten) >= 1 {
        power_of_ten = power_of_ten * 10;
        digits = digits + 1;
    }
    
    // One power greater than needed, so decrease it
    power_of_ten = power_of_ten / 10;
    digits = digits - 2;

    let mut bcd_number: i32 = 0;
    let mut number: i32 = x;
    while digits >= 0 {
        let div_result: i32 = (number / power_of_ten) as i32;
        bcd_number = bcd_number + (div_result << digits*4);
        number = number - div_result * power_of_ten;
        power_of_ten = power_of_ten / 10;
        digits = digits - 1;
    }

    bcd_number
}

fn i32_to_bcd_base10(x: i32) -> i32 {
    let tenths: i32;
    tenths = (x / 10) as i32;
    let ones: i32;
    ones = x - (tenths * 10);
    let shifted_tenths: i32;
    shifted_tenths = tenths << 4;
    shifted_tenths + ones
}

#[cfg(test)]
mod tests {
    use super::i32_to_bcd_base10;
    use super::i32_to_bcd;
    #[test]
    fn i32_to_bcd_base10_test() {
        let x: i32;
        x = 12;

        let result = i32_to_bcd_base10(x);

        assert_eq!(result, 18);
    }
    #[test]
    fn i32_to_bcd_test() {
        let x: i32 = 132;

        let result = i32_to_bcd(x);

        assert_eq!(result, 306);
    }

    #[test]
    fn i32_to_bcd_test_single_digit() {
        let x: i32 = 3;

        let result = i32_to_bcd(x);

        assert_eq!(result, 3);
    }
}
