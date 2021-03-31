
# Defines the memory functions for the core library

module core;

use core.arch.x86_64 if "x86_64";

# Allocate memory
func malloc(size:int) -> int64
    ptr : int64 = 0;
begin
    ptr = syscall(linux_mmap, 0, size, 3, 34, -1, 0);
    return ptr;
end

# Free memory
func free(address:int64, size:int)
begin
    syscall(linux_free, address, size);
end

