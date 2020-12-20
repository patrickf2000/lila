
#OUTPUT
#ffff8234
#aa
#88
#END

#RET 0

extern func puts(s:str)

func main -> int
    x : short = 0x8234
begin
    
    printf("%x\n", x)
    printf("%x\n", 0x00AA)
    
    x = 0x0088
    printf("%x\n", x)
    
    return 0
end
