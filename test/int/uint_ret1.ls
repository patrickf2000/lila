
#OUTPUT
#U1: 10596801
#U2: 11517903
#U3: 20
#END

#RET 0

use std.text_io;

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
    
    printLnStrInt("U1: ", u1);
    printLnStrInt("U2: ", u2);
    printLnStrInt("U3: ", u3);
    
    return 0;
end

