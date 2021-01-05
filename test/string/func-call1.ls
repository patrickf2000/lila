
#OUTPUT
#Hi!
#Hi!
#How are you?
#END

#RET 0

func print_ln(s:str)
begin
    println(s);
end

func main -> int
    s1 : str = "Hi!";
    s2 : str = s1;
begin
    print_ln(s1);
    print_ln(s2);
    print_ln("How are you?");
    
    return 0;
end
