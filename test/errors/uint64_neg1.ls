
#OUTPUT
#Syntax Error: Negation invalid for this type.
# -> [13] x : uint64 = -9
#
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    x : uint64 = -9
begin
    return 0
end

