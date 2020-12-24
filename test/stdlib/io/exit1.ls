
#OUTPUT
#END

#RET 10

use std.arch.x86_64;

func _start
begin
    syscall(linux_exit, 10);
end

