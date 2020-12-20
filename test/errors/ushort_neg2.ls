
#OUTPUT
#Syntax Error: Invalid use of negation operator.
# -> [16] y = -x
#
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    x, y : ushort = 0
begin
    x = 9
    y = -x

    return 0
end

