
#OUTPUT
#Syntax Error: Invalid context- Expecting "begin" before code.
# -> [13] int = 10
#
#END

#RET 1

extern func printf(s:str, ...)

func main -> int
    int = 10
begin
    return 0
end

