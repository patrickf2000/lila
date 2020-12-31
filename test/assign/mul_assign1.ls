
#OUTPUT
#I: 1 | Z: 1
#I: 2 | Z: 2
#I: 3 | Z: 6
#I: 4 | Z: 24
#I: 5 | Z: 120
#I: 6 | Z: 720
#I: 7 | Z: 5040
#I: 8 | Z: 40320
#I: 9 | Z: 362880
#I: 10 | Z: 3628800
#END

#RET 0

extern func printf(s:str, ...)

func main() -> int
    i : int = 1;
    z : int = 1;
begin
    while i <= 10
        z *= i;
        printf("I: %d | Z: %d\n", i, z);
        i += 1;
    end
    
    return 0;
end
