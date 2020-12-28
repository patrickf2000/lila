// This file is part of the Lila compiler
// Copyright (C) 2020 Patrick Flynn
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; version 2.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

// Gets a register based on position
// Kernel argument registers
pub fn riscv64_karg_reg(pos : i32) -> String {
    match pos {
        1 => return "a7".to_string(),
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
pub fn riscv64_arg_reg(pos : i32) -> String {
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

pub fn riscv64_arg_freg(pos : i32) -> String {
    match pos {
        1 => return "fa0".to_string(),
        2 => return "fa1".to_string(),
        3 => return "fa2".to_string(),
        4 => return "fa3".to_string(),
        5 => return "fa4".to_string(),
        6 => return "fa5".to_string(),
        _ => return String::new(),
    };
}

// Operation registers
// S2 and S3 are for internal operations
pub fn riscv64_op_reg(pos : i32) -> String {
    match pos {
        0 => return "s4".to_string(),
        1 => return "s5".to_string(),
        2 => return "s6".to_string(),
        3 => return "s7".to_string(),
        4 => return "s8".to_string(),
        _ => return String::new(),
    };
}

// FS2 and FS3 are for internal operations
pub fn riscv64_op_freg(pos : i32) -> String {
    match pos {
        0 => return "fs4".to_string(),
        1 => return "fs5".to_string(),
        2 => return "fs6".to_string(),
        3 => return "fs7".to_string(),
        4 => return "fs8".to_string(),
        _ => return String::new(),
    };
}
