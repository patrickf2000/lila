
#OUTPUT
#Hello!
#END

#RET 3

func _start
begin
    syscall(1, 1, "Hello!\n", 7)
    syscall(60, 3) 
end

