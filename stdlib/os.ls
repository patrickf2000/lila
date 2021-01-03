
module std;

use core.arch.x86_64 if "x86_64";
use std.arch.riscv64 if "riscv64";

# Change current working directory of process
func chdir(path:str)
begin
    syscall(linux_chdir, path);
end

