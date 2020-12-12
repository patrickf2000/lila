
#OUTPUT
#Syntax Error: Invalid token in expression.
# -> [14] x = =
#
#END

#RET 1

extern func printf(s:str, ...)

func main -> int
    int x = 5
    x = =
    return 0
end

