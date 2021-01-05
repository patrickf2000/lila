
#OUTPUT
#S1: -24143
#S2: -20545
#END

#RET 0

extern func printf(s:str, ...)
extern func puts(s:str)

func short1 -> short
begin
    return 0xA1B1;
end

func short2 -> short
    x : short = 0xAFBF;
begin
    return x;
end

func main -> int
    s1, s2 : short = 0;
begin
    s1 = short1();
    s2 = short2();
    
    printf("S1: %d\n", s1);
    printf("S2: %d\n", s2);
    
    return 0;
end

