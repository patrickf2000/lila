
#OUTPUT
#X: 164
#Y: 85
#END

#RET 0

extern func printf(s:str, ...)

func print_two(x:ubyte, y:ubyte)
begin
    printf("X: %d\n", x);
    printf("Y: %d\n", y);
end

func main -> int
    x : ubyte = 0xA4;
begin
    print_two(x, 0x55);
    
    return 0;
end

