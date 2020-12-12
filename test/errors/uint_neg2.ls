
#OUTPUT
#Syntax Error: Invalid use of negation operator.
# -> [14] uint y = -x
#
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    uint x = 9
    uint y = -x

    return 0
end

