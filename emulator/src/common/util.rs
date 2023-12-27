pub fn is_negative(value: u8) -> bool {
    value & 0b1000_0000 != 0
}

pub fn overflows_positive(result: u16, a: u8, b: u8) -> bool {
    let a_neg = is_negative(a);
    let b_neg = is_negative(b);
    let result_neg = is_negative(result as u8);

    if a_neg == b_neg && a_neg != result_neg {
        return true;
    }

    false
}

pub fn overflows_negative(result: u16, a: u8, b: u8) -> bool {
    let a_neg = is_negative(a);
    let b_neg = is_negative(b);
    let result_neg = is_negative(result as u8);

    if a_neg != b_neg && a_neg != result_neg {
        return true;
    }

    false
}