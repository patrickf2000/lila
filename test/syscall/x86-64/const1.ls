
#OUTPUT
#Hello!
#END

#RET 3

const int linux_write = 1;
const int linux_exit = 60;

const int STDOUT = 1;

func _start
begin
    syscall(linux_write, STDOUT, "Hello!\n", 7);
    syscall(linux_exit, 3);
end
