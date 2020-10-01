
// Gets a register based on position
// Kernel argument registers
pub fn amd64_karg_reg32(pos : i32) -> String {
    match pos {
        1 => return "eax".to_string(),
        2 => return "edi".to_string(),
        3 => return "esi".to_string(),
        4 => return "edx".to_string(),
        _ => return String::new(),
    };
}

pub fn amd64_karg_reg64(pos : i32) -> String {
    match pos {
        1 => return "rax".to_string(),
        2 => return "rdi".to_string(),
        3 => return "rsi".to_string(),
        4 => return "rdx".to_string(),
        _ => return String::new(),
    };
}

// Function argument registers
pub fn amd64_arg_reg32(pos : i32) -> String {
    match pos {
        1 => return "edx".to_string(),
        2 => return "esi".to_string(),
        _ => return String::new(),
    };
}

pub fn amd64_arg_reg64(pos : i32) -> String {
    match pos {
        1 => return "rdi".to_string(),
        2 => return "rsi".to_string(),
        _ => return String::new(),
    };
}

// Operation registers
pub fn amd64_op_reg32(pos : i32) -> String {
    match pos {
        0 => return "eax".to_string(),
        1 => return "ebx".to_string(),
        2 => return "edx".to_string(),
        _ => return String::new(),
    };
}

