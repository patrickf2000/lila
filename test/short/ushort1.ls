
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
    b1 : short = 0xFFF2;
    b2 : ushort = 0xFFF2;
begin
    
    printf("B1: %x\n", b1);
    printf("B2: %x\n", b2);
    
    puts("As decimal:");
    
    printf("B1: %d\n", b1);
    printf("B2: %d\n", b2);
    
    return 0;
end
