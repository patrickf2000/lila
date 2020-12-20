
#OUTPUT
#X: a4
#Y: 55
#END

#RET 0

extern func printf(s:str, ...)
extern func puts(s:str)

func print_two(x:ubyte, y:ubyte)
begin
    printf("X: %x\n", x)
    printf("Y: %x\n", y)
end

func main -> int
    x : ubyte = 0xA4
begin
    print_two(x, 0x55)
    
    return 0
end

