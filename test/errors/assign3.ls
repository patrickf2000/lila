
#OUTPUT
#Syntax Error: Invalid token in expression.
# -> [15] x = =
#
#END

#RET 1

extern func printf(s:str, ...)

func main -> int
    x : int = 5
begin
    x = =
    return 0
end

