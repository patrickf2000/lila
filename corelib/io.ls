
module core;

use core.arch.x86_64 if "x86_64";
use core.string;

# The system exit function
func sys_exit(code:int)
begin
    syscall(linux_exit, code);
end

