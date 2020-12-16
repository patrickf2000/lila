use std::io::{BufWriter, Write};
use std::fs::File;

use parser::ltac::{LtacInstr, LtacType, LtacArg};
use crate::utils::*;

// Builds an extern declaration
pub fn riscv64_build_extern(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    line.push_str(".extern ");
    line.push_str(&code.name);
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[RISCV64_build_extern] Write failed.");
}

// Builds a label
pub fn riscv64_build_label(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    line.push_str(&code.name);
    line.push_str(":\n");
    
    writer.write(&line.into_bytes())
        .expect("[RISCV64_build_label] Write failed.");
}

// Builds a function
pub fn riscv64_build_func(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let stack_size = code.arg1_val + 16;
    let ra = stack_size - 8;
    let s0 = stack_size - 16;

    let mut line = String::new();
    line.push_str(".global ");
    line.push_str(&code.name);
    line.push_str("\n");
    
    line.push_str(&code.name);
    line.push_str(":\n");
    
    line.push_str("  addi sp, sp, -");
    line.push_str(&stack_size.to_string());
    line.push_str("\n");

    line.push_str("  sd ra, ");
    line.push_str(&ra.to_string());
    line.push_str("(sp)\n");

    line.push_str("  sd s0, ");
    line.push_str(&s0.to_string());
    line.push_str("(sp)\n");
    
    line.push_str("  addi s0, sp, ");
    line.push_str(&stack_size.to_string());
    line.push_str("\n\n");

    writer.write(&line.into_bytes())
        .expect("[RISCV64_build_func] Write failed.");
}

// Builds a return statement
pub fn riscv64_build_ret(writer : &mut BufWriter<File>, stack_size : i32) {
    let ra = stack_size - 8;
    let s0 = stack_size - 16;

    let mut line = String::new();

    // Restore the return address and stack pointer
    line.push_str("  ld ra, ");
    line.push_str(&ra.to_string());
    line.push_str("(sp)\n");

    line.push_str("  ld s0, ");
    line.push_str(&s0.to_string());
    line.push_str("(sp)\n");
    
    line.push_str("  addi sp, sp, ");
    line.push_str(&stack_size.to_string());
    line.push_str("\n");

    line.push_str("  jr ra\n\n");

    writer.write(&line.into_bytes())
        .expect("[RISCV64_build_ret] Write failed.");
}

// Builds a load-arg statement
pub fn riscv64_build_ldarg(writer : &mut BufWriter<File>, code : &LtacInstr, stack_top : i32) {
    let mut line = String::new();

    match code.instr_type {
        LtacType::LdArgI32 | LtacType::LdArgU32 => line.push_str("  sw "),

        _ => {},
    }
    
    let reg = riscv64_arg_reg(code.arg2_val);
    line.push_str(&reg);
    line.push_str(", ");

    match code.arg1 {
        LtacArg::Mem(val) => {
            let pos = stack_top - val;
            line.push_str("-");
            line.push_str(&pos.to_string());
            line.push_str("(s0)");
        },

        _ => {},
    }

    line.push_str("\n");

    writer.write(&line.into_bytes())
        .expect("[RISCV64_build_ldarg] Write failed.");
}
