
#OUTPUT
#Syntax Error: Only integers and strings are valid in system calls.
# -> [14] syscall(60, 3.14)
#
#END

#RET 0

extern func printf(s:str, ...)

func main -> int
begin
    syscall(60, 3.14)
    return 0
end

