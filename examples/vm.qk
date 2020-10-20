
# A simple VM example (sort of...)
# Commands:
# A1 -> println
# A2 -> assign var
# A3 -> i.add
# A4 -> i.print
# A5 -> exit

extern func printf(s:str, ...)
extern func open(path:str, flags:int) -> int
extern func close(fd:int)

func decode(op:byte)
    if op == 0xA1
        puts("println")
    elif op == 0xA2
        puts("assign var")
    elif op == 0xA3
        puts("i.add")
    elif op == 0xA4
        puts("i.print")
    elif op == 0xA5
        puts("exit")
    else
        puts("Unknown opcode!")
    end
end

func main -> int
    int fd = open("./prog.bin", 0)
    byte[1] numbers = array
    
    int i = 0
    while i == 0
        read(fd, numbers, 1)
        byte x = numbers[0]
        
        if x == 0
            break
        else
            decode(x)
        end
    end
    
    close(fd)
    
    return 0
end

