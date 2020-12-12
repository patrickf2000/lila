
#OUTPUT
#Syntax Error: Invalid function, constant, or variable name: y
# -> [15] int answer = x * y
#
#END

#RET 0

extern func puts(s:str)

func main -> int
    int x = 6
    
    int answer = x * y
    printf("Answer: %d\n", answer)
    
    return 0
end
