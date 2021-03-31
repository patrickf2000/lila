//
// Copyright 2021 Patrick Flynn
// This file is part of the Ida compiler.
// Ida is licensed under the BSD-3 license. See the COPYING file for more information.
//

use std::io::{BufWriter, Write};
use std::fs::File;

use parser::ltac::{LtacInstr, LtacArg};
use crate::utils::*;

// Builds function/sytem call arguments
pub fn aarch64_build_pusharg(writer : &mut BufWriter<File>, code : &LtacInstr, is_karg : bool, stack_size : i32) {
    let mut reg32 = aarch64_arg_reg32(code.arg2_val);
    let mut reg64 = aarch64_arg_reg64(code.arg2_val);

    if is_karg {
        reg32 = aarch64_karg_reg32(code.arg2_val);
        reg64 = aarch64_karg_reg64(code.arg2_val);
    }

    let mut line = String::new();

    match &code.arg1 {

        LtacArg::Mem(val) => {
            let pos = stack_size - (*val);
        
            line.push_str("  ldr ");
            line.push_str(&reg32);
            line.push_str(", [sp, ");
            line.push_str(&pos.to_string());
            line.push_str("]\n");
        },
        
        LtacArg::I32(val) => {
            line.push_str("  mov ");
            line.push_str(&reg32);
            line.push_str(", ");
            line.push_str(&val.to_string());
            line.push_str("\n");
        },
    
        LtacArg::PtrLcl(ref val) => {
            line.push_str("  adrp ");
            line.push_str(&reg64);
            line.push_str(", ");
            line.push_str(val);
            line.push_str("\n");

            line.push_str("  add ");
            line.push_str(&reg64);
            line.push_str(", ");
            line.push_str(&reg64);
            line.push_str(", :lo12:");
            line.push_str(val);
            line.push_str("\n");
        },

        _ => {},
    }

    writer.write(&line.into_bytes())
        .expect("[AArch64_build_pusharg] Write failed.");
}

// Builds a function call
// Param: name
pub fn aarch64_build_call(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = "  bl ".to_string();
    line.push_str(&code.name);
    line.push_str("\n\n");
    
    writer.write(&line.into_bytes())
        .expect("[AArch64_build_call] Write failed.");
}
