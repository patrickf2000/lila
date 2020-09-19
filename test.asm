STR0 .string "Hello!"
extern puts

global main:
  push rbp
  mov rbp, rsp
  sub rsp, 16

  mov dword [rbp-4], 5
  mov eax, 3
  add eax, [rbp-4]
  mov [rbp-8], eax
  mov rdi, STR0
  call puts


  leave
  ret

