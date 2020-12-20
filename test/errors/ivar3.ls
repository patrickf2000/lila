
#OUTPUT
#Syntax Error: Invalid function, constant, or variable name: y
# -> [16] answer = x * y
#
#END

#RET 0

extern func puts(s:str)

func main -> int
    x : int = 6
    answer : int = 0
begin
    answer = x * y
    printf("Answer: %d\n", answer)
    
    return 0
end
