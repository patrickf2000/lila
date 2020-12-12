
#OUTPUT
#Syntax Error: Negation invalid for this type.
# -> [13] byte x = -0xAB
#
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
    byte x = -0xAB

    return 0
end

