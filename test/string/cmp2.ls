
#OUTPUT
#Equal
#Equal
#Equal
#Not equal
#Not equal
#END

#RET 0

extern func puts(s:str)

func test1
    s1 : str = "Hi!";
begin
    if s1 == "Hi!"
        puts("Equal");
    else
        puts("Not equal");
    end
end

func test2
    s1 : str = "Hi!";
begin
    if "Hi!" == s1
        puts("Equal");
    else
        puts("Not equal");
    end
end

func test3
begin
    if "Hi!" == "Hi!"
        puts("Equal");
    else
        puts("Not equal");
    end
end

func test4
begin
    if "Hi!" == "Yo!"
        puts("Equal");
    else
        puts("Not equal");
    end
end

func test5
    s1 : str = "Hi!";
begin
    if s1 == "Hi, how are you?"
        puts("Equal");
    else
        puts("Not equal");
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

