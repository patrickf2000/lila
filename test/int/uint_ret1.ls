
#OUTPUT
#U1: a1b1c1
#U2: afbfcf
#U3: 20
#END

#RET 0

extern func printf(s:str, ...)
extern func puts(s:str)

func uint1 -> uint
begin
    return 0xA1B1C1;
end

func uint2 -> uint
    x : uint = 0xAFBFCF;
begin
    return x;
end

func uint3 -> uint
begin
    return 20;
end

func main -> int
    u1 : uint = 0;
    u2 : uint = 0;
    u3 : uint = 0;
begin
    u1 = uint1();
    u2 = uint2();
    u3 = uint3();
    
    printf("U1: %x\n", u1);
    printf("U2: %x\n", u2);
    printf("U3: %d\n", u3);
    
    return 0;
end

