
module std

# Calculates absolute value
func abs(num:int) -> int
    if num < 0
        num = num * -1
    end
    
    return num
end

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

