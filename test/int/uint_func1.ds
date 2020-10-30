
#OUTPUT
#X: b5c6
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
    ushort x = 0xA4B5C6
    print_two(x, 0x55AABB)
    
    return 0
end

