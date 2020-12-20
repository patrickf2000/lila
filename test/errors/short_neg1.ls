
#OUTPUT
#Syntax Error: Negation invalid for this type.
# -> [13] x : short = -0xABCD
#
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    x : short = -0xABCD
begin
    return 0
end

