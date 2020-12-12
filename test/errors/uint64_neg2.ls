
#OUTPUT
#Syntax Error: Invalid use of negation operator.
# -> [14] uint64 y = -x
#
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    uint64 x = 9
    uint64 y = -x

    return 0
end

