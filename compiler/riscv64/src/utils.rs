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

// Operation registers
// S2 is for internal operations
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
