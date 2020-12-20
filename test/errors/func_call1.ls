
#OUTPUT
#Syntax Error: Invalid token in expression.
# -> [14] printf(=)
#
#END

#RET 1

extern func printf(s:str, ...)

func main -> int
begin
    printf(=)
    return 0
end

