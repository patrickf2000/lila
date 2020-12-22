
module std;

use std.arch.x86_64;

func num_length(num:int) -> int
    len : int = 0;
begin
    while num != 0
        num = num / 10;
        len = len + 1;
    end
    
    return len;
end

func print_int(num:int)
    length : int = num_length(num);
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

func println_int(num:int)
begin
    print_int(num);
    syscall(linux_write, STDOUT, "\n", 1);
end
