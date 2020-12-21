
#OUTPUT
#Syntax Error: Invalid token in expression.
# -> [15] if x > =
#
#END

#RET 1

extern func printf(s:str, ...)

func main -> int
    x : int = 20;
begin
    if x > =
        printf("%d\n", x)
    end
    
    return 0;
end

