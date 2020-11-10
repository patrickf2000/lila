
use std::io::{BufWriter, Write};
use std::fs::File;

use parser::ltac::{LtacType, LtacInstr, LtacArg};
use crate::utils::*;

// Builds a move-offset instruction
pub fn amd64_build_mov_offset(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    
    // Specific for float literals
    match code.arg2 {
        LtacArg::F32(ref val) => {
            line.push_str("  movss xmm2, DWORD PTR ");
            line.push_str(&val);
            line.push_str("[rip]\n");
        },
        
        LtacArg::F64(ref val) => {
            line.push_str("  movsd xmm2, QWORD PTR ");
            line.push_str(&val);
            line.push_str("[rip]\n");
        },
        
        _ => {},
    }
    
    // Needed if the source is an array index
    if code.instr_type == LtacType::MovOffImm {
        match code.arg2 {
            LtacArg::Mem(pos) => {
                line.push_str("  mov r15, QWORD PTR [rbp-");
                line.push_str(&pos.to_string());
                line.push_str("]\n");
                
                match code.arg1 {
                    LtacArg::Reg8(_p) => line.push_str("  mov r15b, BYTE PTR [r15+"),
                    LtacArg::Reg16(_p) => line.push_str("  mov r15w, WORD PTR [r15+"),
                    LtacArg::Reg64(_p) => line.push_str("  mov r15, QWORD PTR [r15+"),
                    LtacArg::FltReg(_p) => line.push_str("  movss xmm1, DWORD PTR [r15+"),
                    LtacArg::FltReg64(_p) => line.push_str("  movsd xmm1, QWORD PTR [r15+"),
                    _ => line.push_str("  mov r15d, DWORD PTR [r15+"),
                }
                
                line.push_str(&code.arg2_offset.to_string());
                line.push_str("]\n");
            },
            
            _ => {},
        }
    } else if code.arg2_offset > 0 && code.instr_type == LtacType::MovOffMem {
        // Load the variable
        line.push_str("  mov r15d, DWORD PTR [rbp-");
        line.push_str(&code.arg2_offset.to_string());
        line.push_str("]\n");
        
        // Clear flags
        line.push_str("  cdqe\n");
        
        // Load the effective address
        line.push_str("  lea r14, [0+r15*");
        line.push_str(&code.arg2_offset_size.to_string());
        line.push_str("]\n");
        
        // Load the array
        match code.arg2 {
            LtacArg::Mem(pos) => {
                line.push_str("  mov r15, QWORD PTR [rbp-");
                line.push_str(&pos.to_string());
                line.push_str("]\n");
            },
            
            _ => {},
        }
        
        // Add to get the proper offset
        line.push_str("  add r15, r14\n");
        
        // Store
        match &code.arg1 {
            LtacArg::Reg64(_p) => line.push_str("  mov r15, QWORD PTR [r15]\n"),
            LtacArg::FltReg(_p) => line.push_str("  movss xmm1, DWORD PTR [r15]\n"),
            LtacArg::FltReg64(_p) => line.push_str("  movsd xmm1, QWORD PTR [r15]\n"),
            _ => line.push_str("  mov r15d, DWORD PTR [r15]\n"),
        }
    }
    
    // The arguments
    match &code.arg1 {
        LtacArg::Empty => {},
        
        LtacArg::Reg8(pos) => {
            let reg = amd64_op_reg8(*pos);
            
            line.push_str("  mov ");
            line.push_str(&reg);
            line.push_str(", ");
        },
        
        LtacArg::Reg16(pos) => {
            let reg = amd64_op_reg16(*pos);
            
            line.push_str("  mov ");
            line.push_str(&reg);
            line.push_str(", ");
        },
        
        LtacArg::Reg32(pos) => {
            let reg = amd64_op_reg32(*pos);
            
            line.push_str("  mov ");
            line.push_str(&reg);
            line.push_str(", ");
        },
        
        LtacArg::Reg64(pos) => {
            let reg = amd64_op_reg64(*pos);
            
            line.push_str("  mov ");
            line.push_str(&reg);
            line.push_str(", ");
        },
        
        LtacArg::FltReg(pos) => {
            let reg = amd64_op_flt(*pos);
            
            line.push_str("  movss ");
            line.push_str(&reg);
            line.push_str(", ");
        },
        
        LtacArg::FltReg64(pos) => {
            let reg = amd64_op_flt(*pos);
            
            line.push_str("  movsd ");
            line.push_str(&reg);
            line.push_str(", ");
        },
        
        LtacArg::RetRegI32 | LtacArg::RetRegU32 => line.push_str("  mov eax, "),
        LtacArg::RetRegI64 | LtacArg::RetRegU64 => line.push_str("  mov rax, "),
        
        LtacArg::RetRegF32 | LtacArg::RetRegF64 => line.push_str("xmm0, "),
        
        LtacArg::Mem(pos) => {
            if code.arg1_offset > 0 && code.instr_type == LtacType::MovOffImm {
                line.push_str("  mov r15, QWORD PTR [rbp-");
                line.push_str(&pos.to_string());
                line.push_str("]\n");
                
                line.push_str("  add r15, ");
                line.push_str(&code.arg1_offset.to_string());
                line.push_str("\n");
                
                match &code.arg2 {
                    LtacArg::I32(_v) => line.push_str("  mov DWORD PTR "),
                    LtacArg::U32(_v) => line.push_str("  mov DWORD PTR "),
                    LtacArg::I64(_v) => line.push_str("  mov QWORD PTR "),
                    LtacArg::U64(_v) => line.push_str("  mov QWORD PTR "),
                    LtacArg::F32(_v) => line.push_str("  movss DWORD PTR "),
                    LtacArg::F64(_v) => line.push_str("  movsd QWORD PTR "),
                    LtacArg::FltReg(_v) => line.push_str("  movss "),
                    LtacArg::FltReg64(_v) => line.push_str("  movsd "),
                    _ => line.push_str("  mov "),
                };
                line.push_str("[r15], ");
            } else if code.arg1_offset > 0 && code.instr_type == LtacType::MovOffMem {
                // Load the variable
                line.push_str("  mov r15d, DWORD PTR [rbp-");
                line.push_str(&code.arg1_offset.to_string());
                line.push_str("]\n");
                
                // Clear flags
                line.push_str("  cdqe\n");
                
                // Load the effective address
                line.push_str("  lea r14, [0+r15*");
                line.push_str(&code.arg1_offset_size.to_string());
                line.push_str("]\n");
                
                // Load the array
                line.push_str("  mov r15, QWORD PTR [rbp-");
                line.push_str(&pos.to_string());
                line.push_str("]\n");
                
                // Add to get the proper offset
                line.push_str("  add r15, r14\n");
                
                // Now set up for the final move
                match &code.arg2 {
                    LtacArg::Reg8(_v) => line.push_str("  mov BYTE PTR "),
                    LtacArg::I32(_v) => line.push_str("  mov DWORD PTR "),
                    LtacArg::U32(_v) => line.push_str("  mov DWORD PTR "),
                    LtacArg::I64(_v) => line.push_str("  mov QWORD PTR "),
                    LtacArg::U64(_v) => line.push_str("  mov QWORD PTR "),
                    LtacArg::F32(_v) => line.push_str("  movss DWORD PTR "),
                    LtacArg::F64(_v) => line.push_str("  movsd QWORD PTR "),
                    LtacArg::FltReg(_v) => line.push_str("  movss "),
                    LtacArg::FltReg64(_v) => line.push_str("  movsd "),
                    _ => line.push_str("  mov "),
                }
                line.push_str("[r15], ");
            } else {
                match code.arg2 {
                    LtacArg::I32(_v) => line.push_str("  mov DWORD PTR "),
                    LtacArg::U32(_v) => line.push_str("  mov DWORD PTR "),
                    LtacArg::I64(_v) => line.push_str("  mov QWORD PTR "),
                    LtacArg::U64(_v) => line.push_str("  mov QWORD PTR "),
                    _ => line.push_str("  mov "),
                }
                
                line.push_str("[rbp-");
                line.push_str(&pos.to_string());
                line.push_str("], ");
            }
        },
        
        _ => {},
    }
    
    match &code.arg2 {
        LtacArg::Empty => {},
        
        LtacArg::Reg8(pos) => {
            let reg = amd64_op_reg8(*pos);
            line.push_str(&reg);
        },
        
        LtacArg::Reg16(_p) => {},
        
        LtacArg::Reg32(pos) => {
            let reg = amd64_op_reg32(*pos);
            line.push_str(&reg);
        },
        
        LtacArg::Reg64(pos) => {
            let reg = amd64_op_reg64(*pos);
            line.push_str(&reg);
        },
        
        LtacArg::FltReg(pos) | LtacArg::FltReg64(pos) => {
            let reg = amd64_op_flt(*pos);
            line.push_str(&reg);
        },
        
        LtacArg::RetRegI8 | LtacArg::RetRegU8 => line.push_str("eax"),
        LtacArg::RetRegI16 | LtacArg::RetRegU16 => line.push_str("eax"),
        LtacArg::RetRegI32 | LtacArg::RetRegU32 => line.push_str("eax"),
        LtacArg::RetRegI64 | LtacArg::RetRegU64 => line.push_str("rax"),
        
        LtacArg::RetRegF32 | LtacArg::RetRegF64 => line.push_str("xmm0"),
        
        LtacArg::Mem(pos) => {
            if code.instr_type == LtacType::MovOffImm || code.instr_type == LtacType::MovOffMem {
                match &code.arg1 {
                    LtacArg::Reg8(_p) => line.push_str("r15b"),
                    LtacArg::Reg16(_p) => line.push_str("r15w"),
                    LtacArg::Reg64(_p) => line.push_str("r15"),
                    LtacArg::FltReg(_p) | LtacArg::FltReg64(_p) => line.push_str("xmm1"),
                    _ => line.push_str("r15d"),
                }
            } else {
                line.push_str("[rbp-");
                line.push_str(&pos.to_string());
                line.push_str("]");
            }
        },
        
        LtacArg::Byte(val) => {
            line.push_str(" BYTE PTR ");
            line.push_str(&val.to_string());
        },
        
        LtacArg::UByte(val) => {
            line.push_str(" BYTE PTR ");
            line.push_str(&val.to_string());
        },
        
        LtacArg::I16(val) => {
            line.push_str(" WORD PTR ");
            line.push_str(&val.to_string());
        },
        
        LtacArg::U16(val) => {
            line.push_str(" WORD PTR ");
            line.push_str(&val.to_string());
        },
        
        LtacArg::I32(val) => line.push_str(&val.to_string()),
        LtacArg::U32(val) => line.push_str(&val.to_string()),
        
        LtacArg::I64(val) => line.push_str(&val.to_string()),
        LtacArg::U64(val) => line.push_str(&val.to_string()),
        
        LtacArg::F32(_v) | LtacArg::F64(_v) => {
            line.push_str("xmm2");
        },
        
        LtacArg::Ptr(_v) => {},
        LtacArg::PtrLcl(_v) => {},
        
        _ => {},
    }
    
    line.push_str("\n");

    writer.write(&line.into_bytes())
        .expect("[AMD64_writer_instr] Write failed.");
}

