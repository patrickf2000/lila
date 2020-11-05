
# Raise an integer base to a power

extern func printf(s:str, ...)

# Raises the base to a power
func pow(base:int, n:int) -> int
    int result = 1
    
    # Calculate
    while n != 0
        int i = n & 1
        if i != 0
            result = result * base
        end
        
        n = n >> 1
        base = base * base
    end
    
    return result
end

func main -> int
    # 2 ** 4 = 16
    
    int base = 2
    int n = 4
    int result = pow(base, n)
    
    printf("%d ** %d = %d\n", base, n, result)
    
    return 0
end

