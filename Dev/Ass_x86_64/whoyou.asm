
%macro printToTerminal 2
    mov rax, 1
    mov rdi, 1
    mov rsi, %1
    mov rdx, %2
    syscall
%endmacro

%macro readFromTerminal 2
    mov rax, 0
    mov rdi, 1
    mov rsi, %1
    mov rdx, %2
    syscall
%endmacro

global _start         
    ; ##########################
    ; code
    ; ##########################
    section .text


_start: 
    printToTerminal message, length
    
    readFromTerminal user_name, 12
    ;mov rax, 0
    ;mov rdi, 1
    ;mov rsi, user_name
    ;mov rdx, 12
    ;syscall

    printToTerminal user_name, 13
    ;mov rax, 1
    ;mov rdi, 1
    ;mov rsi, user_name
    ;mov rdx, 13
    ;syscall

    ; ##########################
    ; exit(0)
    ; ##########################
    mov rax, 60             ; sys call for exit
    xor rdi, rdi            ; exit code 0
    syscall                 ; invoke os to exit

    ; ##########################
    ; Variables
    ; ##########################
    section .data   
message: db "Hello, what's yer name lass? " ; const char * message = "Hello, World"oxa is "\n"
length: equ $-message;  int length= len(message) =12;
user_name: db 12 dup(0), 0x0a
