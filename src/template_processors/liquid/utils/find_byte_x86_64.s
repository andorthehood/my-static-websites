.text
.globl find_byte_scan
.type find_byte_scan, @function
# size_t find_byte_scan(const uint8_t* ptr, size_t len, uint8_t byte) // returns index or usize::MAX
find_byte_scan:
    # Args: rdi=ptr, rsi=len, rdx=byte
    xor rax, rax              # i = 0
    test rsi, rsi
    je .Lnot_found
.Lloop:
    mov bl, BYTE PTR [rdi + rax]
    cmp bl, dl
    je .Lfound
    inc rax
    cmp rax, rsi
    jb .Lloop
.Lnot_found:
    mov rax, -1               # usize::MAX
    ret
.Lfound:
    ret
.size find_byte_scan, .-find_byte_scan 