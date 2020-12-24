
# Provides common file operations

module std;

use std.string;
use std.io;

func get_char(file:int) -> char
    buf : byte[1] = array;
    c : char = 0;
    code : int = 0;
begin
    code = read(file, buf, 1);
    if code <= 0
        return 0x0;
    end
    
    c = buf[0];
    
    return c;
end

func get_line(file:int) -> str
    line : byte[100] = array;
    i : int = 0;
    c : char = 1;
begin
    while c != 0x0
        c = get_char(file);
        
        if c == 0x0
            break;
        elif c == 0xA
            break;
        end
        
        line[i] = c;
        i = i + 1;
    end
    
    if i == 0
        line[0] = 0x0;
    end
    
    return line;
end

