use std::fs::File;
use std::io::{Write, BufWriter};

use parser::ltac::{LtacInstr, LtacType, LtacArg};
use crate::utils::*;

// Builds a load/store instruction
pub fn aarch64_build_ld_str(writer : &mut BufWriter<File>, code : &LtacInstr, stack_size : i32) {
    let mut line : String;
    let pos = stack_size - code.arg1_val;
    
    match &code.instr_type {
        LtacType::Ld => line = "  ldr ".to_string(),
        LtacType::LdB => line = "  ldrb ".to_string(),
        LtacType::StrB => line = "  strb ".to_string(),
        LtacType::Str => line = "  str ".to_string(),
        
        _ => line = String::new(),
    }
    
    match &code.arg2_type {
        LtacArg::Reg8(pos) | LtacArg::Reg32(pos) => {
            let reg = aarch64_op_reg32(*pos);
            line.push_str(&reg);
        },
        
        LtacArg::Reg64 => {
            let reg = aarch64_op_reg64(code.arg2_val);
            line.push_str(&reg);
        },
        
        LtacArg::RetRegI32 => line.push_str("w0"),
        LtacArg::RetRegI64 => line.push_str("x0"),
        
        _ => {},
    }
    
    line.push_str(", [sp, ");
    line.push_str(&pos.to_string());
    line.push_str("]\n");
    
    writer.write(&line.into_bytes())
        .expect("[AARCH64_build_ld_str] Write failed.");
}

// Builds the store-pointer instruction
pub fn aarch64_build_strptr(writer : &mut BufWriter<File>, code : &LtacInstr, stack_size : i32) {
    let mut line = String::new();
    let pos = stack_size - code.arg1_val;
    
    line.push_str("  adrp x4, ");
    line.push_str(&code.arg2_sval);
    line.push_str("\n");
    
    line.push_str("  add x4, x4, :lo12:");
    line.push_str(&code.arg2_sval);
    line.push_str("\n");
    
    line.push_str("  str x4, [sp, ");
    line.push_str(&pos.to_string());
    line.push_str("]\n\n");
    
    writer.write(&line.into_bytes())
        .expect("[AARCH64_build_strptr] Write failed.");
}

// A common function for data moves
pub fn aarch64_build_mov(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = "  mov ".to_string();
    
    match &code.arg1_type {
        LtacArg::Reg8(pos) | LtacArg::Reg32(pos) => {
            let reg = aarch64_op_reg32(*pos);
        
            line.push_str(&reg);
            line.push_str(", ");
        },
        
        LtacArg::Reg64 => {
            let reg = aarch64_op_reg64(code.arg2_val);
            line.push_str(&reg);
        },
        
        LtacArg::RetRegI32 => line.push_str("w0, "),
        LtacArg::RetRegI64 => line.push_str("x0, "),
        
        _ => {},
    }
    
    match &code.arg2_type {
        LtacArg::Reg32(pos) => {
            let reg = aarch64_op_reg32(*pos);
            line.push_str(&reg);
        },
        
        LtacArg::RetRegI32 => line.push_str("w0"),
        LtacArg::RetRegI64 => line.push_str("x0"),
        
        LtacArg::Byte(val) => line.push_str(&val.to_string()),
        LtacArg::I32(val) => line.push_str(&val.to_string()),
        
        _ => {},
    }
    
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[AARCH64_build_mov] Write failed.");
}

/*pub fn aarch64_build_mov(writer : &mut BufWriter<File>, code : &LtacInstr, stack_size : i32) {
    let mut line = "".to_string();
    
    // Check if we're storing to a variable
    if code.arg1_type == LtacArg::Mem {
        let mut reg = "w4".to_string();
    
        match &code.arg2_type {
            LtacArg::Reg => {
                reg = aarch64_op_reg32(code.arg2_val);
            },
            
            LtacArg::RetRegI32 => {
                reg = "w0".to_string();
            },
            
            LtacArg::RetRegI64 => {
                reg = "x0".to_string();
            },
            
            LtacArg::Mem => {},
            
            LtacArg::I32 => {
                line.push_str("  mov w4, ");
                line.push_str(&code.arg2_val.to_string());
                line.push_str("\n");
            },
            
            LtacArg::Ptr => {
                line.push_str("  adrp x4, ");
                line.push_str(&code.arg2_sval);
                line.push_str("\n");
                
                line.push_str("  add x4, x4, :lo12:");
                line.push_str(&code.arg2_sval);
                line.push_str("\n");
                
                reg = "x4".to_string();
            },
            
            _ => {},
        }
        
        let pos = stack_size - code.arg1_val;
        
        line.push_str("  str ");
        line.push_str(&reg);
        line.push_str(", [sp, ");
        line.push_str(&pos.to_string());
        line.push_str("]\n");
        
    // Check if we are loading a variable
    } else if code.arg2_type == LtacArg::Mem {
        let pos = stack_size - code.arg2_val;
        
        match &code.arg1_type {
            LtacArg::Reg => {
                let reg = aarch64_op_reg32(code.arg1_val);
            
                line.push_str("  ldr ");
                line.push_str(&reg);
                line.push_str(", [sp, ");
                line.push_str(&pos.to_string());
                line.push_str("]\n");
            },
            
            LtacArg::RetRegI32 => {
                line.push_str("  ldr w0, [sp, ");
                line.push_str(&pos.to_string());
                line.push_str("]\n");
            },
            
            _ => {},
        }
        
    // Otherwise, we're probably moving something to a register
    } else {
        match &code.arg1_type {
            LtacArg::Reg => {
                let reg = aarch64_op_reg32(code.arg1_val);
                
                line.push_str("  mov ");
                line.push_str(&reg);
                line.push_str(", ");
            },
            
            LtacArg::RetRegI32 => {
                line.push_str("  mov w0, ");
            },
            
            _ => {},
        }
        
        match &code.arg2_type {
            LtacArg::Reg => {
                let reg = aarch64_op_reg32(code.arg2_val);
                
                line.push_str(&reg);
                line.push_str("\n");
            },
        
            LtacArg::RetRegI32 => {
                line.push_str("w0\n");
            },
        
            LtacArg::I32 => {
                line.push_str(&code.arg2_val.to_string());
                line.push_str("\n");
            },
            
            LtacArg::Ptr => {},
            
            _ => {},
        }
    }
    
    writer.write(&line.into_bytes())
        .expect("[AARCH64_build_mov] Write failed.");
}*/

// A move with offset instruction
pub fn aarch64_build_mov_offset(writer : &mut BufWriter<File>, code : &LtacInstr, stack_size : i32) {
    let mut line = String::new();
    let mut dest_reg = "w5".to_string();
    
    match &code.arg1_type {
        LtacArg::Reg32(pos) => {
            dest_reg = aarch64_op_reg32(*pos);
        },
        
        LtacArg::Reg64 => {},
        
        LtacArg::RetRegI32 => {},
        LtacArg::RetRegI64 => {},
        
        // Load to x6
        LtacArg::Mem => {
            let pos = stack_size - code.arg1_val;
            
            if code.instr_type == LtacType::MovOffImm {
                line.push_str("  ldr x6, [sp, ");
                line.push_str(&pos.to_string());
                line.push_str("]\n");
                
                line.push_str("  add x6, x6, ");
                line.push_str(&code.arg1_offset.to_string());
                line.push_str("\n");
            } else if code.instr_type == LtacType::MovOffMem {
                let index_pos = stack_size - code.arg1_offset;
            
                // Load the variable to x7
                // Then load the array as above
                line.push_str("  ldrsw x7, [sp, ");
                line.push_str(&index_pos.to_string());
                line.push_str("]\n");
                
                line.push_str("  lsl x7, x7, 2\n");
                
                line.push_str("  ldr x6, [sp, ");
                line.push_str(&pos.to_string());
                line.push_str("]\n");
                
                line.push_str("  add x6, x6, x7\n");
            }
        },
        
        _ => {},
    }
    
    // Whatever happens here should go to x5
    match &code.arg2_type {
        LtacArg::Reg32(pos) => {
            dest_reg = aarch64_op_reg32(*pos);
        },
        
        LtacArg::Reg64 => {},
        
        LtacArg::RetRegI32 => {},
        LtacArg::RetRegI64 => {},
        
        LtacArg::Mem => {
            let pos = stack_size - code.arg2_val;
            
            if code.instr_type == LtacType::MovOffImm {
                line.push_str("  ldr x6, [sp, ");
                line.push_str(&pos.to_string());
                line.push_str("]\n");
                
                line.push_str("  ldr ");
                line.push_str(&dest_reg);
                line.push_str(", [x6, ");
                line.push_str(&code.arg2_offset.to_string());
                line.push_str("]\n");
            } else if code.instr_type == LtacType::MovOffMem {
                let index_pos = stack_size - code.arg2_offset;
            
                // Load the variable to x7
                // Then load the array as above
                line.push_str("  ldrsw x7, [sp, ");
                line.push_str(&index_pos.to_string());
                line.push_str("]\n");
                
                line.push_str("  lsl x7, x7, 2\n");
                
                line.push_str("  ldr x6, [sp, ");
                line.push_str(&pos.to_string());
                line.push_str("]\n");
                
                line.push_str("  add x6, x6, x7\n");
                
                // ldr <dest>, [x6]
                line.push_str("  ldr ");
                line.push_str(&dest_reg);
                line.push_str(", [x6]\n");
            }
        },
        
        LtacArg::I32(val) => {
            line.push_str("  mov ");
            line.push_str(&dest_reg);
            line.push_str(", ");
            line.push_str(&val.to_string());
            line.push_str("\n");
        },
        
        _ => {},
    }
    
    // Store back
    if code.arg1_type == LtacArg::Mem {
        line.push_str("  str ");
        line.push_str(&dest_reg);
        line.push_str(", [x6]\n");
    }
    
    writer.write(&line.into_bytes())
        .expect("[AARCH64_build_mov_offset] Write failed.");
}

