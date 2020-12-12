
#OUTPUT
#Syntax Error: Invalid token in expression.
# -> [13] int x = =
#
#END

#RET 1

extern func printf(s:str, ...)

func main -> int
    int x = =
    return 0
end

