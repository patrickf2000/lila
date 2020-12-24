
module std;

use std.arch.x86_64;
use std.string;
use std.fs;

func numLength(num:int) -> int
    len : int = 0;
begin
    while num != 0
        num = num / 10;
        len = len + 1;
    end
    
    return len;
end

func printInt(num:int)
    length : int = numLength(num);
    x : int = length - 1;
    digit : int = 0;
    b_digit : byte = 0;
    number : byte[length] = array;
begin
    while num != 0
        digit = num % 10;
        num = num / 10;
        b_digit = digit + '0';
        number[x] = b_digit;
        x = x - 1;
    end
    
    syscall(linux_write, STDOUT, number, length);
end

func printLnInt(num:int)
begin
    print_int(num);
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
        c = get_char(0);
        
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
       b = get_char(0);
       
       if b == 0x0
           break;
       end
       
       b = b - 48;
       result = result * 10;
       result = result + b;
    end
    
    return result;
end
