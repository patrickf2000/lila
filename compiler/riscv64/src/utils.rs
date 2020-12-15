
// Gets a register based on position
// Kernel argument registers
pub fn riscv64_karg_reg32(pos : i32) -> String {
    match pos {
        1 => return "a8".to_string(),
        2 => return "a0".to_string(),
        3 => return "a1".to_string(),
        4 => return "a2".to_string(),
        5 => return "a3".to_string(),
        6 => return "a4".to_string(),
        7 => return "a5".to_string(),
        _ => return String::new(),
    };
}

// TODO: This is probably wrong
pub fn riscv64_karg_reg64(pos : i32) -> String {
    match pos {
        1 => return "a8".to_string(),
        2 => return "a0".to_string(),
        3 => return "a1".to_string(),
        4 => return "a2".to_string(),
        5 => return "a3".to_string(),
        6 => return "a4".to_string(),
        7 => return "a5".to_string(),
        _ => return String::new(),
    };
}

// Function argument registers
pub fn riscv64_arg_reg32(pos : i32) -> String {
    match pos {
        1 => return "a0".to_string(),
        2 => return "a1".to_string(),
        3 => return "a2".to_string(),
        4 => return "a3".to_string(),
        5 => return "a4".to_string(),
        6 => return "a5".to_string(),
        _ => return String::new(),
    };
}

pub fn riscv64_arg_reg64(pos : i32) -> String {
    match pos {
        1 => return "a0".to_string(),
        2 => return "a1".to_string(),
        3 => return "a2".to_string(),
        4 => return "a3".to_string(),
        5 => return "a4".to_string(),
        6 => return "a5".to_string(),
        _ => return String::new(),
    };
}

// Operation registers
pub fn riscv64_op_reg32(pos : i32) -> String {
    match pos {
        0 => return "s2".to_string(),
        1 => return "s3".to_string(),
        2 => return "s4".to_string(),
        3 => return "s5".to_string(),
        4 => return "s6".to_string(),
        _ => return String::new(),
    };
}

/*pub fn aarch64_op_reg64(pos : i32) -> String {
    match pos {
        0 => return "x9".to_string(),
        1 => return "x10".to_string(),
        2 => return "x11".to_string(),
        3 => return "x12".to_string(),
        4 => return "x13".to_string(),
        _ => return String::new(),
    };
}*/
