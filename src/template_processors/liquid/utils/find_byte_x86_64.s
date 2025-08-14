.text
.globl find_byte_scan
.type find_byte_scan, @function
# size_t find_byte_scan(const uint8_t* ptr, size_t len, uint8_t byte) // returns index or usize::MAX
find_byte_scan:
    # Args: rdi=ptr, rsi=len, rdx=byte
    xor rax, rax              # i = 0
    test rsi, rsi
    je .Lfind_byte_not_found
.Lfind_byte_loop:
    mov bl, BYTE PTR [rdi + rax]
    cmp bl, dl
    je .Lfind_byte_found
    inc rax
    cmp rax, rsi
    jb .Lfind_byte_loop
.Lfind_byte_not_found:
    mov rax, -1               # usize::MAX
    ret
.Lfind_byte_found:
    ret
.size find_byte_scan, .-find_byte_scan 