
use parser::ltac;
use parser::ltac::{LtacFile, LtacType, LtacArg};

fn is_move(instr : &LtacType) -> bool {
    match instr {
        LtacType::MovUW |
        LtacType::Mov | LtacType::MovU |
        LtacType::MovQ | LtacType::MovUQ
            => return true,
            
        _ => return false,
    }
}

fn has_mem(arg : &LtacArg) -> bool {
    match arg {
        LtacArg::Mem(_n) => return true,
        LtacArg::MemOffsetImm(_n1, _n2) => return true,
        LtacArg::MemOffsetMem(_n1, _n2, _n3) |
        LtacArg::MemOffsetReg(_n1, _n2, _n3) => return true,
        
        _ => return false,
    }
}

// Returns the proper load instruction for a given move
fn load_for_mov(instr : &LtacType) -> LtacType {
    match instr {
        LtacType::I64Add | LtacType::I64Sub | LtacType::I64Mul |
        LtacType::I64Div | LtacType::I64Mod => return LtacType::LdQ,
        LtacType::U64Add | LtacType::U64Mul |
        LtacType::U64Div | LtacType::U64Mod => return LtacType::LdUQ,
        LtacType::MovU => return LtacType::LdU,
        LtacType::MovQ => return LtacType::LdQ,
        LtacType::MovUQ => return LtacType::LdUQ,
        _ => return LtacType::Ld,
    }
}

// Returns the proper store instruction for a given move
fn store_for_mov(instr : &LtacType) -> LtacType {
    match instr {
        LtacType::MovU => return LtacType::StrU,
        LtacType::MovQ => return LtacType::StrQ,
        LtacType::MovUQ => return LtacType::StrUQ,
        _ => return LtacType::Str,
    }
}

// Returns a register for a given move statement
fn reg_for_mov(instr : &LtacType, pos : i32) -> LtacArg {
    match instr {
        LtacType::LdQ | LtacType::LdUQ |
        LtacType::MovQ | LtacType::MovUQ => return LtacArg::Reg64(pos),
        _ => return LtacArg::Reg32(pos),
    }
}

// The main RISC optimizer loop
pub fn risc_optimize(file : &LtacFile) -> Result<LtacFile, ()> {
    let mut file2 = LtacFile {
        name : file.name.clone(),
        data : file.data.clone(),
        code : Vec::new(),
    };
    
    let code = file.code.clone();
    
    for line in code.iter() {
        let mut instr2 = line.clone();
        
        if is_move(&line.instr_type) {
            if has_mem(&line.arg1) {
                let instr_type = store_for_mov(&line.instr_type);
                let mut store = ltac::create_instr(instr_type);
                store.arg1 = instr2.arg1.clone();
                store.arg2 = reg_for_mov(&line.instr_type, 3);
                
                instr2.arg1 = reg_for_mov(&line.instr_type, 3);
                
                file2.code.push(instr2);
                file2.code.push(store);
            } else if has_mem(&line.arg2) {
                let instr_type = load_for_mov(&line.instr_type);
                let mut load = ltac::create_instr(instr_type);
                load.arg1 = instr2.arg2.clone();
                load.arg2 = reg_for_mov(&line.instr_type, 3);
                
                instr2.arg2 = reg_for_mov(&line.instr_type, 3);
                
                file2.code.push(load);
                file2.code.push(instr2);
            } else {
                file2.code.push(instr2);
            }
        } else {
            if has_mem(&line.arg2) && line.instr_type != LtacType::PushArg {
                let instr_type = load_for_mov(&line.instr_type);
                let mut load = ltac::create_instr(instr_type.clone());
                load.arg1 = instr2.arg2.clone();
                load.arg2 = reg_for_mov(&instr_type, 3);
                
                instr2.arg2 = reg_for_mov(&instr_type, 3);
                
                file2.code.push(load);
            }
            
            file2.code.push(instr2);
        }
    }
    
    Ok(file2)
}

