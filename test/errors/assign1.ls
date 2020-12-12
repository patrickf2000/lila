
#OUTPUT
#Syntax Error: Invalid assignment or call.
# -> [14] x + 2 * 3
#
#END

#RET 1

extern func printf(s:str, ...)

func main -> int
    int x = 10
    x + 2 * 3
    return 0
end

