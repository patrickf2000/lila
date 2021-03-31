
#OUTPUT
#2 ** 4 = 16
#END

#RET 0

extern func printf(s:str, ...)

# Raises the base to a power
func pow(base:int, n:int) -> int
    result : int = 1;
    i : int = 0;
begin
  
    # Calculate
    while n != 0
        i = n & 1;
        if i != 0
            result = result * base;
        end
        
        n = n >> 1;
        base = base * base;
    end
    
    return result;
end

# 2 ** 4 = 16
func main -> int
    base   : int = 2;
    n      : int = 4;
    result : int = 0;
begin
    result = pow(base, n);
    
    printf("%d ** %d = %d\n", base, n, result);
    
    return 0;
end

