
#OUTPUT
#Equal
#Not equal
#END

#RET 0

func test1
    s1 : str = "Hi!";
    s2 : str = "Hi!";
begin
    if s1 == s2
        println("Equal");
    else
        println("Not equal");
    end
end

func test2
    s1 : str = "Hi!";
    s2 : str = "Hi, how are you?";
begin
    if s1 == s2
        println("Equal");
    else
        println("Not equal");
    end
end

func main -> int
begin
    test1();
    test2();
    return 0;
end

