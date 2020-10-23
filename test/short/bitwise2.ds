
#OUTPUT
#X = 4
#x & 5 = 4
#x | 5 = 5
#x ^ 5 = 1
#x << 2 = 10
#x >> 2 = 1
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    ushort x = 4
    
    ushort a1 = x & 5
    ushort a2 = x | 5
    ushort a3 = x ^ 5
    ushort a4 = x << 2
    ushort a5 = x >> 2
    
    printf("X = %d\n", x)
    printf("x & 5 = %x\n", a1)
    printf("x | 5 = %x\n", a2)
    printf("x ^ 5 = %x\n", a3)
    printf("x << 2 = %x\n", a4)
    printf("x >> 2 = %x\n", a5)
    
    return 0
end

