
#OUTPUT
#Syntax Error: Invalid constant or variable name: answerrr
# -> [18] printf("Answer: %d\n", answerrr);
#
#END

#RET 0

extern func puts(s:str)

func main -> int
    x : int = 6;
    y : int = 3;
    answer : int = 0;
begin
    answer = x * y;
    printf("Answer: %d\n", answerrr);
    
    return 0;
end
