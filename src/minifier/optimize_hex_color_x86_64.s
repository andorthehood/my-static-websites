.text
.globl optimize_hex_color_scan
.type optimize_hex_color_scan, @function
# size_t optimize_hex_color_scan(const uint8_t* ptr, size_t len, uint8_t* can_shorten)
# Returns: number of hex chars consumed (0-6), sets *can_shorten to 1 if 6 chars can be shortened
optimize_hex_color_scan:
    # Arguments: rdi=ptr, rsi=len, rdx=can_shorten
    push rbp
    mov rbp, rsp
    
    xor rax, rax              # count = 0
    mov BYTE PTR [rdx], 0     # *can_shorten = false initially
    
    # Check if we have enough length for at least one character
    test rsi, rsi
    jz .Ldone
    
    # Buffer to store hex chars (on stack)
    sub rsp, 8                # allocate 8 bytes for hex_chars
    mov r8, rsp               # r8 = hex_chars buffer
    
    # Collect up to 6 hex characters
    xor r9, r9                # r9 = index into buffer
    
.Lcollect_loop:
    cmp r9, 6                 # if index >= 6, stop
    jge .Lcheck_shortening
    cmp rax, rsi              # if count >= len, stop
    jge .Lcheck_shortening
    
    # Load next character
    mov bl, BYTE PTR [rdi + rax]
    
    # Check if it's a hex digit
    call .Lis_hex_digit
    test al, al
    jz .Lcheck_shortening     # not hex, stop collecting
    
    # Store hex char in buffer and continue
    mov BYTE PTR [r8 + r9], bl
    inc r9
    mov rax, r9               # update return count
    jmp .Lcollect_loop
    
.Lcheck_shortening:
    # If we collected exactly 6 chars, check if they can be shortened
    cmp r9, 6
    jne .Lcleanup
    
    # Check if chars[0] == chars[1] && chars[2] == chars[3] && chars[4] == chars[5]
    mov bl, BYTE PTR [r8]     # chars[0]
    cmp bl, BYTE PTR [r8 + 1] # chars[1]
    jne .Lcleanup
    
    mov bl, BYTE PTR [r8 + 2] # chars[2]
    cmp bl, BYTE PTR [r8 + 3] # chars[3]
    jne .Lcleanup
    
    mov bl, BYTE PTR [r8 + 4] # chars[4]
    cmp bl, BYTE PTR [r8 + 5] # chars[5]
    jne .Lcleanup
    
    # All pairs match, can shorten
    mov BYTE PTR [rdx], 1     # *can_shorten = true
    
.Lcleanup:
    add rsp, 8                # deallocate buffer
    
.Ldone:
    mov rsp, rbp
    pop rbp
    ret

# Helper function: check if byte in bl is hex digit
# Returns 1 in al if hex digit, 0 otherwise
.Lis_hex_digit:
    # Check '0' <= bl <= '9'
    cmp bl, '0'
    jb .Lnot_hex
    cmp bl, '9'
    jbe .Lis_hex
    
    # Check 'A' <= bl <= 'F'
    cmp bl, 'A'
    jb .Lnot_hex
    cmp bl, 'F'
    jbe .Lis_hex
    
    # Check 'a' <= bl <= 'f'
    cmp bl, 'a'
    jb .Lnot_hex
    cmp bl, 'f'
    jbe .Lis_hex
    
.Lnot_hex:
    xor al, al                # return 0
    ret
    
.Lis_hex:
    mov al, 1                 # return 1
    ret

.size optimize_hex_color_scan, .-optimize_hex_color_scan