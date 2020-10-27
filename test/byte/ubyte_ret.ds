
#OUTPUT
#B1: a1
#B2: af
#END

#RET 0

extern func printf(s:str, ...)
extern func puts(s:str)

func byte1 -> ubyte
    return 0xA1
end

func byte2 -> ubyte
    ubyte x = 0xAF
    return x
end

func main -> int
    ubyte b1 = byte1()
    ubyte b2 = byte2()
    
    printf("B1: %x\n", b1)
    printf("B2: %x\n", b2)
    
    return 0
end

