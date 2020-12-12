
#OUTPUT
#U1: a1b1c1
#U2: afbfcf
#U3: 20
#END

#RET 0

extern func printf(s:str, ...)
extern func puts(s:str)

func uint1 -> uint
    return 0xA1B1C1
end

func uint2 -> uint
    uint x = 0xAFBFCF
    return x
end

func uint3 -> uint
    return 20
end

func main -> int
    uint u1 = uint1()
    uint u2 = uint2()
    uint u3 = uint3()
    
    printf("U1: %x\n", u1)
    printf("U2: %x\n", u2)
    printf("U3: %d\n", u3)
    
    return 0
end

