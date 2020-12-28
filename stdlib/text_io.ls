
module std;

use std.arch.x86_64;
use std.string;
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

func numLength(num:int) -> int
    len : int = 0;
begin
    if num < 0
        len = 1;
    end
    
    while num != 0
        num = num / 10;
        len = len + 1;
    end
    
    return len;
end

func check_neg(num:int) -> int
begin
    if num < 0
        num = num * -1;
    end
    
    return num;
end

func getHexLength(num:int) -> int
    len : int = 0;
begin
    while num > 15
        len = len + 1;
        num = num / 16;
    end
    
    len = len + 1;
    
    return len;
end

func getHexDigit(digit:int) -> byte
begin
    if digit == 1
        return '1';
    elif digit == 2
        return '2';
    elif digit == 3
        return '3';
    elif digit == 4
        return '4';
    elif digit == 5
        return '5';
    elif digit == 6
        return '6';
    elif digit == 7
        return '7';
    elif digit == 8
        return '8';
    elif digit == 9
        return '9';
    elif digit == 10
        return 'a';
    elif digit == 11
        return 'b';
    elif digit == 12
        return 'c';
    elif digit == 13
        return 'd';
    elif digit == 14
        return 'e';
    elif digit == 15
        return 'f';
    end
    
    return '0';
end

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

func printLnStrInt(s:str, num:int)
begin
    print(s);
    printInt(num);
    syscall(linux_write, STDOUT, "\n", 1);
end

func printLnStrHex(s:str, num:int)
begin
    print(s);
    printHex(num);
    syscall(linux_write, STDOUT, "\n", 1);
end

func printLnInt(num:int)
begin
    printInt(num);
    syscall(linux_write, STDOUT, "\n", 1);
end

func printLnHex(num:int)
begin
    printHex(num);
    syscall(linux_write, STDOUT, "\n", 1);
end

func printLn(s:str)
    length : int = strlen(s);
begin
    syscall(linux_write, STDOUT, s, length);
    syscall(linux_write, STDOUT, "\n", 1);
end

func print(s:str)
    length : int = strlen(s);
begin
    syscall(linux_write, STDOUT, s, length);
end

func printLn2(s1:str, s2:str)
    s1_len : int = strlen(s1);
    s2_len : int = strlen(s2);
begin
    syscall(linux_write, STDOUT, s1, s1_len);
    syscall(linux_write, STDOUT, s2, s2_len);
    syscall(linux_write, STDOUT, "\n", 1);
end

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
