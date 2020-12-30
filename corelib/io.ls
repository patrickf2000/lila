
module core;

use core.arch.x86_64 if "x86_64";
use core.string;

# The two internal print functions
func print(line:str)
    length : int = strlen(line);
begin
    syscall(linux_write, STDOUT, line, length);
end

func println(line:str)
begin
    print(line);
    syscall(linux_write, STDOUT, "\n", 1);
end

# The system exit function
func sys_exit(code:int)
begin
    syscall(linux_exit, code);
end

