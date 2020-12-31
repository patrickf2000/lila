
#OUTPUT
#1) 1
#1) 2
#1) 3
#1) 4
#1) 5
#2) 3
#2) 4
#2) 5
#2) 6
#2) 7
#END

#RET 0

# The test basically divides all values of i by 2
# If the number is odd, 1 is output

extern func printf(s:str, ...)

func print_numbers(numbers:int[], stage:int)
    i, z : int = 0;
begin
    while i < 5
        z = numbers[i];
        printf("%d) %d\n", stage, z);
        i++;   
    end
end

func main() -> int
    i, z : int = 0;
    numbers : int[5];
begin
    # Set things up
    while i < 5
        numbers[i] = i + 1;
        i++;
    end
    
    print_numbers(numbers, 1);   
    
    # Now the test
    i = 0;
    while i < 5
        numbers[i] += 2;
        i++;
    end
    
    print_numbers(numbers, 2);
    
    return 0;
end

