
#OUTPUT
#B1: ffffffa1
#B2: ffffffaf
#END

#RET 0

extern func printf(s:str, ...)
extern func puts(s:str)

func byte1 -> byte
    return 0xA1
end

func byte2 -> byte
    byte x = 0xAF
    return x
end

func main -> int
    byte b1 = byte1()
    byte b2 = byte2()
    
    printf("B1: %x\n", b1)
    printf("B2: %x\n", b2)
    
    return 0
end

