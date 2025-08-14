.text
.globl is_ascii_whitespace_scan
.type is_ascii_whitespace_scan, @function
# uint8_t is_ascii_whitespace_scan(uint8_t byte) // returns 1 if whitespace, 0 if not
is_ascii_whitespace_scan:
    # Args: rdi=byte (only low 8 bits used)
    mov al, dil               # move byte to al
    
    # Check for space (32)
    cmp al, 32
    je .Lis_whitespace
    
    # Check for tab (9)
    cmp al, 9
    je .Lis_whitespace
    
    # Check for newline (10)
    cmp al, 10
    je .Lis_whitespace
    
    # Check for carriage return (13)
    cmp al, 13
    je .Lis_whitespace
    
    # Check for form feed (12)
    cmp al, 12
    je .Lis_whitespace
    
    # Check for vertical tab (11)
    cmp al, 11
    je .Lis_whitespace
    
    # Not whitespace, return 0
    xor rax, rax
    ret

.Lis_whitespace:
    mov rax, 1                # return 1 for whitespace
    ret
.size is_ascii_whitespace_scan, .-is_ascii_whitespace_scan 