
#OUTPUT
#S1: a1b1
#S2: afbf
#END

#RET 0

extern func printf(s:str, ...)
extern func puts(s:str)

func short1 -> ushort
    return 0xA1B1
end

func short2 -> ushort
    ushort x = 0xAFBF
    return x
end

func main -> int
    ushort s1 = short1()
    ushort s2 = short2()
    
    printf("S1: %x\n", s1)
    printf("S2: %x\n", s2)
    
    return 0
end

