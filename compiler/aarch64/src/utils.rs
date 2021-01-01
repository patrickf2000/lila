
// This file is part of the Lila compiler
// Copyright (C) 2020-2021 Patrick Flynn
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
pub fn aarch64_karg_reg32(pos : i32) -> String {
    match pos {
        1 => return "w8".to_string(),
        2 => return "w0".to_string(),
        3 => return "w1".to_string(),
        4 => return "w2".to_string(),
        5 => return "w3".to_string(),
        6 => return "w4".to_string(),
        7 => return "w5".to_string(),
        _ => return String::new(),
    };
}

pub fn aarch64_karg_reg64(pos : i32) -> String {
    match pos {
        1 => return "x8".to_string(),
        2 => return "x0".to_string(),
        3 => return "x1".to_string(),
        4 => return "x2".to_string(),
        5 => return "x3".to_string(),
        6 => return "x4".to_string(),
        7 => return "x5".to_string(),
        _ => return String::new(),
    };
}

// Function argument registers
pub fn aarch64_arg_reg32(pos : i32) -> String {
    match pos {
        1 => return "w0".to_string(),
        2 => return "w1".to_string(),
        3 => return "w2".to_string(),
        4 => return "w3".to_string(),
        5 => return "w4".to_string(),
        6 => return "w5".to_string(),
        _ => return String::new(),
    };
}

pub fn aarch64_arg_reg64(pos : i32) -> String {
    match pos {
        1 => return "x0".to_string(),
        2 => return "x1".to_string(),
        3 => return "x2".to_string(),
        4 => return "x3".to_string(),
        5 => return "x4".to_string(),
        6 => return "x5".to_string(),
        _ => return String::new(),
    };
}

// Operation registers
pub fn aarch64_op_reg32(pos : i32) -> String {
    match pos {
        0 => return "w9".to_string(),
        1 => return "w10".to_string(),
        2 => return "w11".to_string(),
        3 => return "w12".to_string(),
        4 => return "w13".to_string(),
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
