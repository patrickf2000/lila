
#OUTPUT
#Syntax Error: Negation invalid for this type.
# -> [13] x : byte = -0xAB
#
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    x : byte = -0xAB
begin
    return 0
end

