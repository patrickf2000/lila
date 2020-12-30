
module std;

use core.arch.x86_64 if "x86_64";
use std.arch.riscv64 if "riscv64";

use std.string;
use std.text_utils;
use std.fs;

# The printf function
func printf(fmt:str, arg1:int64, arg2:int64, arg3:int64, arg4:int64, arg5:int64)
    args : int64[5] = array;
    c : char = 0;
    i, length : int = 0;
    buf : byte[1] = array;
    
    arg_index : int = 0;
    i64_arg : int64 = 0;
    i_arg : int = 0;
    c_arg : char = 0;
    s_arg : str = "";
begin
    args[0] = arg1;
    args[1] = arg2;
    args[2] = arg3;
    args[3] = arg4;
    args[4] = arg5;
    
    # First, determine the length
    length = strlen(fmt);
    
    while i < length
        c = fmt[i];
        
        if c == '%'
            i = i + 1;
            c = fmt[i];
            
            i64_arg = args[arg_index];
            
            if c == 'd'
                i_arg = i64_arg;
                printInt(i_arg);
            elif c == 'x'
                i_arg = i64_arg;
                printHex(i_arg);
            elif c == 'c'
                c_arg = i64_arg;
                buf[0] = c_arg;
                syscall(linux_write, STDOUT, buf, 1);
            elif c == 's'
                print(i64_arg);
            else
                buf[0] = 37;
                syscall(linux_write, STDOUT, buf, 1);
                
                buf[0] = c;
                syscall(linux_write, STDOUT, buf, 1);
                
                i = i + 1;
                continue;
            end
            
            arg_index = arg_index + 1;
            i = i + 1;
        elif c == '\'
            if c == 'n'
                syscall(linux_write, STDOUT, "\n", 1);
            
                i = i + 1;
            end
            
            i = i + 1;
        else
            buf[0] = c;
            syscall(linux_write, STDOUT, buf, 1);
            
            i = i + 1;
        end
    end
end

# Prints a number as a hex number
func printHex(num:int)
    length : int = getHexLength(num);
    x : int = length - 1;
    digit : int = 0;
    b_digit : byte = 0;
    number : byte[length] = array;
begin
    if num == 0
        syscall(linux_write, STDOUT, "0", 1);
    elif num <= 15
        b_digit = getHexDigit(num);
        number[0] = b_digit;
        
        syscall(linux_write, STDOUT, number, 1);
        return;
    else
        while num > 15
            digit = num % 16;
            num = num / 16;
            
            b_digit = getHexDigit(digit);
            number[x] = b_digit;
            x = x - 1;
        end
        
        b_digit = getHexDigit(num);
        number[x] = b_digit;
        
        syscall(linux_write, STDOUT, number, length);
    end
end

# Print an integer
func printInt(n:int)
    num : int = check_neg(n);
    length : int = numLength(num);
    x : int = length - 1;
    digit, is_neg : int = 0;
    b_digit : byte = 0;
    number : byte[length] = array;
begin
    if num == 0
        syscall(linux_write, STDOUT, "0", 1);
    else
        if n < 0
            syscall(linux_write, STDOUT, "-", 1);
        end
        
        while num != 0
            digit = num % 10;
            num = num / 10;
            b_digit = digit + '0';
            number[x] = b_digit;
            x = x - 1;
        end
        
        syscall(linux_write, STDOUT, number, length);
    end
end

# Read a line of text from std input
func readLn() -> str
    line : byte[100] = array;
    c : char = 0;
    index : int = 0;
begin
    while c != 0xA
        c = getChar(0);
        
        if c == 0xA
            break;
        elif c == 0x0
            break;
        end
        
        line[index] = c;
        index = index + 1;
    end
    
    return line;
end

# Read an integer from std input
func readInt() -> int
    result : int = 0;
    b : byte = 0x0;
begin
    while b != 0x0
       b = getChar(0);
       
       if b == 0x0
           break;
       end
       
       b = b - 48;
       result = result * 10;
       result = result + b;
    end
    
    return result;
end

