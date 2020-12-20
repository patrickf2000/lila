
#OUTPUT
#45
#aa
#ffffff88
#END

#RET 0

extern func puts(s:str)

func main -> int
    x : byte = 0x45
begin
    
    printf("%x\n", x)
    printf("%x\n", 0xAA)
    
    x = 0x88
    printf("%x\n", x)
    
    return 0
end
