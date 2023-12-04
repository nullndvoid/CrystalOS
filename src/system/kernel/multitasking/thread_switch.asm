
.intel_syntax noprefix
    pushfq

    mov rax, rsp
    mov rsp, rdi

    mov rdi, rax
    call add_paused_thread

    popfq
    ret