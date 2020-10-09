
// Function argument registers
pub fn aarch64_arg_reg32(pos : i32) -> String {
    match pos {
        1 => "w0".to_string(),
        2 => "w1".to_string(),
        3 => "w2".to_string(),
        4 => "w3".to_string(),
        5 => "w4".to_string(),
        6 => "w5".to_string(),
        7 => "w6".to_string(),
        8 => "w7".to_string(),
        _ => String::new(),
    }
}

pub fn aarch64_arg_reg64(pos : i32) -> String {
    match pos {
        1 => "x0".to_string(),
        2 => "x1".to_string(),
        3 => "x2".to_string(),
        4 => "x3".to_string(),
        5 => "x4".to_string(),
        6 => "x5".to_string(),
        7 => "x6".to_string(),
        8 => "x7".to_string(),
        _ => String::new(),
    }
}

// Kernel argument registers
pub fn aarch64_karg_reg32(pos : i32) -> String {
    match pos {
        1 => "w8".to_string(),
        2 => "w0".to_string(),
        3 => "w1".to_string(),
        4 => "w2".to_string(),
        5 => "w3".to_string(),
        6 => "w4".to_string(),
        7 => "w5".to_string(),
        _ => String::new(),
    }
}

pub fn aarch64_karg_reg64(pos : i32) -> String {
    match pos {
        1 => "x8".to_string(),
        2 => "x0".to_string(),
        3 => "x1".to_string(),
        4 => "x2".to_string(),
        5 => "x3".to_string(),
        6 => "x4".to_string(),
        7 => "x5".to_string(),
        _ => String::new(),
    }
}

// Operation registers
// We can use x9-x15
// x4 and x5 are for internal uses
pub fn aarch64_op_reg32(pos : i32) -> String {
    match pos {
        0 => "w9".to_string(),
        1 => "w10".to_string(),
        2 => "w11".to_string(),
        3 => "w12".to_string(),
        4 => "w13".to_string(),
        5 => "w14".to_string(),
        6 => "w15".to_string(),
        _ => String::new(),
    }
}

