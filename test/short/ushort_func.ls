
#OUTPUT
#X: 42165
#Y: 21930
#END

#RET 0

extern func printf(s:str, ...)

func print_two(x:ushort, y:ushort)
begin
    printf("X: %d\n", x);
    printf("Y: %d\n", y);
end

func main -> int
    x : ushort = 0xA4B5;
begin
    print_two(x, 0x55AA);
    
    return 0;
end

