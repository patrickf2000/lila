
#OUTPUT
#Syntax Error: Expected '=' in array assignment.
# -> [15] numbers[1] 55
#
#END

#RET 1

extern func printf(s:str, ...)

func main -> int
    numbers : int[10] = array
begin
    numbers[1] 55
    
    return 0
end

