
#OUTPUT
#20
#60
#60
#END

#RET 0

const int linux_exit = 60

extern func printf(s:str, ...)

func main -> int
    int code = 20
    int val = linux_exit
    
    printf("%d\n", code)
    printf("%d\n", val)
    printf("%d\n", linux_exit)
    
    return 0
end
