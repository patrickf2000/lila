
#OUTPUT
#2.300000
#4.600000
#6.900000
#9.200000
#11.500000
#13.800000
#16.100000
#18.400000
#20.700000
#23.000000
#Done
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    double[10] numbers = array
    
    int i = 0
    double x = 1.0
    while i < 10
        numbers[i] = x * 2.3
        
        x = x + 1.0
        i = i + 1
    end
    
    i = 0
    while i < 10
        double x = numbers[i]
        printf("%f\n", x)
        
        i = i + 1
    end
    
    printf("Done\n")
    
    return 0
end

