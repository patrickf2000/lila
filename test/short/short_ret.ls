
#OUTPUT
#S1: ffffa1b1
#S2: ffffafbf
#END

#RET 0

extern func printf(s:str, ...)
extern func puts(s:str)

func short1 -> short
begin
    return 0xA1B1
end

func short2 -> short
    x : short = 0xAFBF
begin
    return x
end

func main -> int
    s1, s2 : short = 0
begin
    s1 = short1()
    s2 = short2()
    
    printf("S1: %x\n", s1)
    printf("S2: %x\n", s2)
    
    return 0
end

