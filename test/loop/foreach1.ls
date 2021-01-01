
#OUTPUT
#2
#3
#4
#5
#6
#END

#RET 0

# The test basically divides all values of i by 2
# If the number is odd, 1 is output

extern func printf(s:str, ...)

func main() -> int
    z, j : int = 0;
    numbers : int[5];
begin
    # Set things up
    for i in 0 .. 5
        numbers[i] = i + 2;
    end
    
    for i in numbers
        printf("%d\n", i);
    end
    
    return 0;
end

