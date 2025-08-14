.text
.globl trim_quotes_scan
.type trim_quotes_scan, @function
# void trim_quotes_scan(const uint8_t* ptr, size_t len, size_t* out_start, size_t* out_end)
trim_quotes_scan:
    push rbp
    mov rbp, rsp
    # Arguments: rdi=ptr, rsi=len, rdx=out_start, rcx=out_end
    xor rax, rax          # start = 0
    mov r8, rsi           # end = len

    # Leading trim loop: while start < end and byte at [ptr+start] is quote
.Llead_loop:
    cmp r8, 0
    je .Ltrail_check
    mov bl, BYTE PTR [rdi + rax]
    cmp bl, 0x22          # '"'
    je .Linc_start
    cmp bl, 0x27          # '\''
    jne .Ltrail_check
.Linc_start:
    inc rax
    cmp rax, r8
    jb .Llead_loop

.Ltrail_check:
    # Trailing trim loop: while end > start and byte at [ptr+end-1] is quote
.Ltrail_loop:
    cmp r8, rax
    jbe .Lwrite_out
    mov bl, BYTE PTR [rdi + r8 - 1]
    cmp bl, 0x22
    je .Ldec_end
    cmp bl, 0x27
    jne .Lwrite_out
.Ldec_end:
    dec r8
    jmp .Ltrail_loop

.Lwrite_out:
    mov QWORD PTR [rdx], rax   # *out_start = start
    mov QWORD PTR [rcx], r8    # *out_end = end
    mov rsp, rbp
    pop rbp
    ret
.size trim_quotes_scan, .-trim_quotes_scan 