
#OUTPUT
#B1: fffffff2
#B2: fff2
#As decimal:
#B1: -14
#B2: 65522
#END

#RET 0

extern func printf(s:str, ...)
extern func puts(s:str)

func main -> int
    short b1 = 0xFFF2
    ushort b2 = 0xFFF2
    
    printf("B1: %x\n", b1)
    printf("B2: %x\n", b2)
    
    puts("As decimal:")
    
    printf("B1: %d\n", b1)
    printf("B2: %d\n", b2)
    
    return 0
end
