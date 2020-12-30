
# Provides access to several common IO system calls

module std;

const int O_RDWR = 2;

# Equivalent to 644.
const int PERM_RW = 420;

# For lseek
const int SEEK_SET = 0;
const int SEEK_CUR = 1;
const int SEEK_END = 2;

use core.arch.x86_64 if "x86_64";
use std.arch.riscv64 if "riscv64";

func open(path:str) -> int
    fd : int = 0;
begin
    fd = syscall(linux_open, path, O_RDWR, PERM_RW);
    return fd;
end

func create(path:str) -> int
    fd : int = 0;
begin
    fd = syscall(linux_create, path, PERM_RW);
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

