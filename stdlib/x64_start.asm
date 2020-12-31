# For now, we just call the main function and
# pass parameters
.intel_syntax noprefix

.text
.globl _start
.extern main

_start:
    xor ebp, ebp
    mov esi, DWORD PTR [rsp+0]
    lea rdi, 8[rsp]
    call main
    
    mov rdi, rax
    mov rax, 60
    syscall
