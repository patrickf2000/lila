
# Provides access to several common UNIX system calls

module std

use std.arch.x86_64

func open(path:str) -> int
    int fd = syscall(linux_open, path, 0)
    return fd
end

func read(fd:int, buf:byte[], size:int)
    syscall(linux_read, fd, buf, size)
end

func lseek(fd:int, offset:int64, whence:int)
    syscall(linux_lseek, fd, offset, whence)
end

func close(fd:int)
    syscall(linux_close, fd)
end

