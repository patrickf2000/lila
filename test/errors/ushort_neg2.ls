
#OUTPUT
#Syntax Error: Invalid use of negation operator.
# -> [14] ushort y = -x
#
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    ushort x = 9
    ushort y = -x

    return 0
end

