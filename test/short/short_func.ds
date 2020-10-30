
#OUTPUT
#X: ffffa4b5
#Y: 55aa
#END

#RET 0

extern func printf(s:str, ...)
extern func puts(s:str)

func print_two(x:short, y:short)
    printf("X: %x\n", x)
    printf("Y: %x\n", y)
end

func main -> int
    short x = 0xA4B5
    print_two(x, 0x55AA)
    
    return 0
end

