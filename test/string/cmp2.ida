
#OUTPUT
#Equal
#Equal
#Equal
#Not equal
#Not equal
#END

#RET 0

func test1
    s1 : str = "Hi!";
begin
    if s1 == "Hi!"
        println("Equal");
    else
        println("Not equal");
    end
end

func test2
    s1 : str = "Hi!";
begin
    if "Hi!" == s1
        println("Equal");
    else
        println("Not equal");
    end
end

func test3
begin
    if "Hi!" == "Hi!"
        println("Equal");
    else
        println("Not equal");
    end
end

func test4
begin
    if "Hi!" == "Yo!"
        println("Equal");
    else
        println("Not equal");
    end
end

func test5
    s1 : str = "Hi!";
begin
    if s1 == "Hi, how are you?"
        println("Equal");
    else
        println("Not equal");
    end
end

func main -> int
begin
    test1();
    test2();
    test3();
    test4();
    test5();
    return 0;
end

