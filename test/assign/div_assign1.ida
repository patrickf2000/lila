
#OUTPUT
#I: 1 | Z: 0
#I: 2 | Z: 1
#I: 3 | Z: 1
#I: 4 | Z: 2
#I: 5 | Z: 2
#I: 6 | Z: 3
#I: 7 | Z: 3
#I: 8 | Z: 4
#I: 9 | Z: 4
#I: 10 | Z: 5
#END

#RET 0

# The test basically divides all values of i by 2

extern func printf(s:str, ...)

func main() -> int
    i : int = 1;
    z : int = 1;
begin
    while i <= 10
        z = i;
        z /= 2;
        printf("I: %d | Z: %d\n", i, z);
        i += 1;
    end
    
    return 0;
end
