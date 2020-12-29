
# Defines the system call numbers for Linux x86-64

module core.arch;

const int linux_read = 0;
const int linux_write = 1;
const int linux_open = 2;
const int linux_close = 3;
const int linux_lseek = 8;
const int linux_mmap = 9;
const int linux_free = 11;
const int linux_exit = 60;
const int linux_create = 85;

const int STDOUT = 1;
const int STDIN = 1;

