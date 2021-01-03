
module std;

use core.arch.x86_64 if "x86_64";
use std.arch.riscv64 if "riscv64";

# Change current working directory of process
func chdir(path:str)
begin
    syscall(linux_chdir, path);
end

# Fork a process
func fork() -> int64
    pid : int64 = 0;
begin
    pid = syscall(linux_fork);
    return pid;
end

# Run a command
func exec_process(cmd:str, args:str[]) -> int
    count : int = sizeof(args);
    new_count : int = count + 2;
    all_args : str[new_count];
    i2 : int = 1;
begin
    all_args[0] = cmd;
    
    for i in 0 .. count
        all_args[i2] = args[i];
        i2++;
    end
    
    all_args[i2] = 0;
    
    syscall(linux_exec, cmd, all_args, 0);
    return 1;
end

