
#OUTPUT
#Hi!
#Hi!
#How are you?
#END

#RET 0

extern func puts(s:str)

func println(s:str)
begin
    puts(s);
end

func main -> int
    s1 : str = "Hi!";
    s2 : str = s1;
begin
    println(s1);
    println(s2);
    println("How are you?");
    
    return 0;
end
