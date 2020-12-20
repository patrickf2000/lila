
#OUTPUT
#Syntax Error: Invalid string variable.
# -> [14] s2 : str = s100
#
#END

#RET 0

extern func puts(s:str)

func main -> int
    s1 : str = "Hello!"
    s2 : str = s100
begin
    puts(s1)
    puts(s2)
    
    return 0
end
