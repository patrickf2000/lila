
# A simple basic interpreter
# Valid commands:
# var x = 5
# print x
# exit

use std.text_io;
use std.string;

# String functions
func findFirst(s:str, c:char) -> int
    length : int = strlen(s);
    c2 : char = 0;
begin
    for i in 0 .. length
        c2 = s[i];
        
        if c2 == c
            return i;
        end
    end
    
    return length;
end

func getSecond(s:str) -> str
    length : int = strlen(s);
    found, pos : int = 0;
    
    c : char = 0;
    
    buf : byte[length];
    default : str = "";
begin
    for i in 0 .. length
        c = s[i];
        
        if c == ' '
            if found == 0
                found = 1;
                continue;
            end
        end
        
        if found == 1
            buf[pos] = c;
            pos++;
        end
    end
    
    if found == 0
        return default;
    end
    
    return buf;
end

# Parses a print statement
func parsePrint(input:str)
    c1, c2 : char = 0;
    length : int = strlen(input);
begin
    c1 = input[0];
    c2 = input[length-1];
    
    if c1 == '"'
        if c2 != '"'
            println("Invalid syntax");
            return;
        end
        
        length -= 1;
        
        for i in 1 .. length
            c1 = input[i];
            printf("%c", c1);
        end
        
        println("");
    else
        println("VAR");
    end
end

# The command parse function
func parse(input:str)
    cmd_end : int = findFirst(input, ' ');
    args : str = getSecond(input);
    buf : byte[cmd_end];
    cmd : str = "";
    c : char = 0;
begin
    for i in 0 .. cmd_end
        buf[i] = input[i];
    end
    
    cmd = buf;
    
    if cmd == "var"
        println("New var");
    elif cmd == "print"
        parsePrint(args);
    else
        printf("Invalid command: %s\n", input);
    end
    
    println("");
end

func run
    cont : int = 1;
    input : str = "";
begin
    while cont == 1
        print("> ");
        
        input = readLn();
        
        if input == "exit"
            return;
        end
        
        parse(input);
    end
end

func main -> int
begin
    run();
    println("Goodbye!");

    return 0;
end

