fn int_to_bcd(x: i32) -> i32 {
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
    use super::int_to_bcd;
    #[test]
    fn it_works() {
        let x: i32;
        x = 12;

        let result = int_to_bcd(x);

        assert_eq!(18, result);
    }
}
