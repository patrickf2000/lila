
#OUTPUT
#X: a4b5c6d7
#Y: 55aabb
#END

#RET 0

extern func printf(s:str, ...)
extern func puts(s:str)

func print_two(x:uint, y:uint)
    printf("X: %x\n", x)
    printf("Y: %x\n", y)
end

func main -> int
    uint x = 0xA4B5C6D7
    print_two(x, 0x55AABB)
    
    return 0
end

