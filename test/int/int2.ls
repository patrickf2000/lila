
#OUTPUT
#11223344
#aabbccd
#11118888
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    x : int = 0x11223344;
begin
    printf("%x\n", x);
    printf("%x\n", 0xAABBCCD);
    
    x = 0x11118888;
    printf("%x\n", x);
    
    return 0;
end
