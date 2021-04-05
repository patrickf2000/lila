//
// Copyright 2021 Patrick Flynn
// This file is part of the Ida compiler.
// Ida is licensed under the BSD-3 license. See the COPYING file for more information.
//

use parser::ltac::{LtacInstr, LtacType, LtacArg};
use crate::asm::*; 

// Builds a load or store instruction
pub fn arm64_build_ld_str(code : &mut Vec<Arm64Instr>, instr : &LtacInstr, stack_size : i32) {
    let mut line : Arm64Instr;
    let is_store : bool;
    
    match instr.instr_type {
        LtacType::Str | LtacType::StrQ => {
            line = create_arm64_instr(Arm64Type::Str);
            is_store = true;
        },
        
        LtacType::Ld => {
            line = create_arm64_instr(Arm64Type::Ldr);
            is_store = false;
        },
        
        _ => return,
    }
    
    // Build store arguments
    if is_store {
        match instr.arg1 {
            LtacArg::Mem(val) => line.arg2 = Arm64Arg::Mem(Arm64Reg::SP, stack_size - val),
            
            LtacArg::MemOffsetImm(pos, offset) => {
                let mut line1 = create_arm64_instr(Arm64Type::Ldr);
                line1.arg1 = Arm64Arg::Reg(Arm64Reg::X0);
                line1.arg2 = Arm64Arg::Mem(Arm64Reg::SP, stack_size - pos);
                code.push(line1);
                
                let mut line2 = create_arm64_instr(Arm64Type::Add);
                line2.arg1 = Arm64Arg::Reg(Arm64Reg::X0);
                line2.arg2 = line2.arg1.clone();
                line2.arg3 = Arm64Arg::Imm32(offset);
                code.push(line2);
                
                line.arg2 = Arm64Arg::RegRef(Arm64Reg::X0);
            },
            
            LtacArg::MemOffsetMem(pos, var, size) => {
                //ldrsw x0, [sp, var]
                let mut line1 = create_arm64_instr(Arm64Type::LdrSW);
                line1.arg1 = Arm64Arg::Reg(Arm64Reg::X0);
                line1.arg2 = Arm64Arg::Mem(Arm64Reg::SP, stack_size - var);
                code.push(line1);
                
                //lsl x0, x0, sqrt(size)
                let mut line2 = create_arm64_instr(Arm64Type::Lsl);
                line2.arg1 = Arm64Arg::Reg(Arm64Reg::X0);
                line2.arg2 = line2.arg1.clone();
                line2.arg3 = Arm64Arg::Imm32(size / 2);
                code.push(line2);
                
                //ldr x1, [sp, pos]
                let mut line3 = create_arm64_instr(Arm64Type::Ldr);
                line3.arg1 = Arm64Arg::Reg(Arm64Reg::X1);
                line3.arg2 = Arm64Arg::Mem(Arm64Reg::SP, stack_size - pos);
                code.push(line3);
                
                // add x0, x0, x1
                let mut line4 = create_arm64_instr(Arm64Type::Add);
                line4.arg1 = Arm64Arg::Reg(Arm64Reg::X0);
                line4.arg2 = line4.arg1.clone();
                line4.arg3 = Arm64Arg::Reg(Arm64Reg::X1);
                code.push(line4);
                
                // Final store
                line.arg2 = Arm64Arg::RegRef(Arm64Reg::X0);
            },
            
            _ => return,
        }
        
        match instr.arg2 {
            LtacArg::Reg8(val) | LtacArg::Reg16(val)
            | LtacArg::Reg32(val) => line.arg1 = Arm64Arg::Reg(arm64_arg_reg32(val)),
            
            LtacArg::Reg64(val) => line.arg1 = Arm64Arg::Reg(arm64_arg_reg64(val)),
            
            LtacArg::RetRegI64 | LtacArg::RetRegU64 => line.arg1 = Arm64Arg::Reg(Arm64Reg::X0),
            
            _ => return,
        }
    // Build load arguments
    } else {
        match instr.arg1 {
            LtacArg::Reg8(val) | LtacArg::Reg16(val)
            | LtacArg::Reg32(val) => line.arg1 = Arm64Arg::Reg(arm64_arg_reg32(val)),
            
            LtacArg::Reg64(val) => line.arg1 = Arm64Arg::Reg(arm64_arg_reg64(val)),
            
            LtacArg::RetRegI8 | LtacArg::RetRegU8
            | LtacArg::RetRegI16 | LtacArg::RetRegU16
            | LtacArg::RetRegI32 | LtacArg::RetRegU32 => {
                line.arg1 = Arm64Arg::Reg(Arm64Reg::W0);
            },
            
            LtacArg::RetRegI64 | LtacArg::RetRegU64 => line.arg1 = Arm64Arg::Reg(Arm64Reg::X0),
            
            _ => return,
        }
        
        match instr.arg2 {
            LtacArg::Mem(val) => line.arg2 = Arm64Arg::Mem(Arm64Reg::SP, stack_size - val),
            
            LtacArg::MemOffsetImm(pos, offset) => {
                let mut line1 = create_arm64_instr(Arm64Type::Ldr);
                line1.arg1 = Arm64Arg::Reg(Arm64Reg::X0);
                line1.arg2 = Arm64Arg::Mem(Arm64Reg::SP, stack_size - pos);
                code.push(line1);
            
                line.arg2 = Arm64Arg::Mem(Arm64Reg::X0, offset);
            },
            
            LtacArg::MemOffsetMem(pos, var, size) => {
                //ldrsw x0, [sp, var]
                let mut line1 = create_arm64_instr(Arm64Type::LdrSW);
                line1.arg1 = Arm64Arg::Reg(Arm64Reg::X0);
                line1.arg2 = Arm64Arg::Mem(Arm64Reg::SP, stack_size - var);
                code.push(line1);
                
                //lsl x0, x0, sqrt(size)
                let mut line2 = create_arm64_instr(Arm64Type::Lsl);
                line2.arg1 = Arm64Arg::Reg(Arm64Reg::X0);
                line2.arg2 = line2.arg1.clone();
                line2.arg3 = Arm64Arg::Imm32(size / 2);
                code.push(line2);
                
                //ldr x1, [sp, pos]
                let mut line3 = create_arm64_instr(Arm64Type::Ldr);
                line3.arg1 = Arm64Arg::Reg(Arm64Reg::X1);
                line3.arg2 = Arm64Arg::Mem(Arm64Reg::SP, stack_size - pos);
                code.push(line3);
                
                // add x0, x0, x1
                let mut line4 = create_arm64_instr(Arm64Type::Add);
                line4.arg1 = Arm64Arg::Reg(Arm64Reg::X0);
                line4.arg2 = line4.arg1.clone();
                line4.arg3 = Arm64Arg::Reg(Arm64Reg::X1);
                code.push(line4);
                
                // Final store
                line.arg2 = Arm64Arg::RegRef(Arm64Reg::X0);
            },
            
            _ => return,
        }
    }
    
    code.push(line);
}

// Builds a move instruction
pub fn arm64_build_mov(code : &mut Vec<Arm64Instr>, instr : &LtacInstr) {
    let mut mov = create_arm64_instr(Arm64Type::Mov);
    mov.arg1 = arm64_build_operand(&instr.arg1, code, false);
    mov.arg2 = arm64_build_operand(&instr.arg2, code, false);
    code.push(mov);
}

// Builds common 3-operand instructions
pub fn arm64_build_instr(code : &mut Vec<Arm64Instr>, instr : &LtacInstr) {
    let mut ln : Arm64Instr;
    
    match instr.instr_type {
        LtacType::I32Add => ln = create_arm64_instr(Arm64Type::Add),
        LtacType::I32Sub => ln = create_arm64_instr(Arm64Type::Sub),
        LtacType::I32Mul => ln = create_arm64_instr(Arm64Type::Mul),
        LtacType::I32Div => ln = create_arm64_instr(Arm64Type::SDiv),
        LtacType::I32Mod => ln = create_arm64_instr(Arm64Type::SDiv),
        
        LtacType::And => ln = create_arm64_instr(Arm64Type::And),
        LtacType::Or => ln = create_arm64_instr(Arm64Type::Orr),
        LtacType::Xor => ln = create_arm64_instr(Arm64Type::Eor),
        LtacType::Lsh => ln = create_arm64_instr(Arm64Type::Lsl),
        LtacType::Rsh => ln = create_arm64_instr(Arm64Type::Lsr),
        
        LtacType::I32Cmp => ln = create_arm64_instr(Arm64Type::Cmp),
        
        _ => return,
    }
    
    let mut separate_li = false;
    if ln.instr_type != Arm64Type::Add && ln.instr_type != Arm64Type::Sub {
        separate_li = true;
    }
    
    ln.arg1 = arm64_build_operand(&instr.arg1, code, false);
    
    if ln.instr_type == Arm64Type::Cmp {
        ln.arg2 = arm64_build_operand(&instr.arg2, code, separate_li);
    } else {
        ln.arg2 = ln.arg1.clone();
        ln.arg3 = arm64_build_operand(&instr.arg2, code, separate_li);
    }
    
    if instr.instr_type == LtacType::I32Mod {
        let dest2 = Arm64Arg::Reg(Arm64Reg::W1);
        ln.arg1 = dest2;
        code.push(ln.clone());
        
        let mut ln2 = create_arm64_instr(Arm64Type::MSub);
        ln2.arg1 = ln.arg2.clone();
        ln2.arg2 = ln.arg1.clone();
        ln2.arg3 = ln.arg3.clone();
        ln2.arg4 = ln.arg2.clone();
        code.push(ln2);
    } else {
        code.push(ln);
    }
}

// Translates an operand
fn arm64_build_operand(arg : &LtacArg, code : &mut Vec<Arm64Instr>, separate_li : bool) -> Arm64Arg {
    let mut to_ret = match arg {
        LtacArg::Reg8(val) | LtacArg::Reg16(val)
        | LtacArg::Reg32(val) => Arm64Arg::Reg(arm64_arg_reg32(*val)),
        
        LtacArg::RetRegI8 | LtacArg::RetRegU8
        | LtacArg::RetRegI16 | LtacArg::RetRegU16
        | LtacArg::RetRegI32 | LtacArg::RetRegU32 => {
            Arm64Arg::Reg(Arm64Reg::W0)
        },
        
        LtacArg::RetRegI64 | LtacArg::RetRegU64 => Arm64Arg::Reg(Arm64Reg::X0),
        
        LtacArg::I32(val) => Arm64Arg::Imm32(*val),
        LtacArg::U32(val) => Arm64Arg::Imm32(*val as i32),
        
        _ => Arm64Arg::Empty,
    };
    
    if separate_li {
        let mut mov = create_arm64_instr(Arm64Type::Mov);
        mov.arg1 = Arm64Arg::Reg(Arm64Reg::W0);
        mov.arg2 = to_ret;
        code.push(mov);
        
        to_ret = Arm64Arg::Reg(Arm64Reg::W0);
    }
    
    to_ret
}

// Returns a register given a numerical value
fn arm64_arg_reg32(val : i32) -> Arm64Reg {
    match val {
        0 => Arm64Reg::W9,
        1 => Arm64Reg::W10,
        2 => Arm64Reg::W11,
        3 => Arm64Reg::W12,
        4 => Arm64Reg::W13,
        5 => Arm64Reg::W14,
        6 => Arm64Reg::W15,
        7 => Arm64Reg::W16,
        _ => Arm64Reg::W17,
    }
}

fn arm64_arg_reg64(val : i32) -> Arm64Reg {
    match val {
        0 => Arm64Reg::X9,
        1 => Arm64Reg::X10,
        2 => Arm64Reg::X11,
        3 => Arm64Reg::X12,
        4 => Arm64Reg::X13,
        5 => Arm64Reg::X14,
        6 => Arm64Reg::X15,
        7 => Arm64Reg::X16,
        _ => Arm64Reg::X17,
    }
}

// Translates a jump instruction
pub fn arm64_build_jump(code : &mut Vec<Arm64Instr>, instr : &LtacInstr) {
    let mut ln : Arm64Instr;
    
    match instr.instr_type {
        LtacType::Br => ln = create_arm64_instr(Arm64Type::B),
        LtacType::Be => ln = create_arm64_instr(Arm64Type::Beq),
        LtacType::Bne => ln = create_arm64_instr(Arm64Type::Bne),
        LtacType::Bl => ln = create_arm64_instr(Arm64Type::Bl),
        LtacType::Ble => ln = create_arm64_instr(Arm64Type::Ble),
        LtacType::Bg => ln = create_arm64_instr(Arm64Type::Bg),
        LtacType::Bge => ln = create_arm64_instr(Arm64Type::Bge),
        
        _ => return,
    }
    
    ln.name = instr.name.clone();
    code.push(ln);
}
