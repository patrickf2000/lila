
# Provides access to several common IO system calls

module std;

use std.arch.x86_64;

func open(path:str) -> int
    fd : int = 0;
begin
    fd = syscall(linux_open, path, 2);
    return fd;
end

func create(path:str) -> int
    fd : int = 0;
begin
    fd = syscall(linux_create, path, 2);
    return fd;
end

func read(fd:int, buf:byte[], size:int) -> int
    code : int = 0;
begin
    code = syscall(linux_read, fd, buf, size);
    return code;
end

func write(fd:int, buf:byte[], size:int) -> int
    code : int = 0;
begin
    code = syscall(linux_write, fd, buf, size);
    return code;
end

func lseek(fd:int, offset:int64, whence:int)
begin
    syscall(linux_lseek, fd, offset, whence);
end

func close(fd:int)
begin
    syscall(linux_close, fd);
end

func sys_exit(code:int)
begin
    syscall(linux_exit, code);
end
