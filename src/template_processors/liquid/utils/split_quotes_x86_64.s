.text
.globl split_quotes_scan
.type split_quotes_scan, @function
# size_t split_quotes_scan(const uint8_t* ptr, size_t len, size_t* splits, size_t max_splits)
# Returns number of comma positions found (not inside quotes)
# splits array will contain the positions of commas that can be used for splitting
split_quotes_scan:
    # Args: rdi=ptr, rsi=len, rdx=splits array, rcx=max_splits
    push rbp
    mov rbp, rsp
    push rbx
    push r12
    push r13
    push r14
    push r15
    
    xor rax, rax              # split_count = 0
    xor r8, r8                # i = 0 (current position)
    xor r9, r9                # in_quotes = false
    xor r10, r10              # quote_char = 0
    
    test rsi, rsi
    je .Ldone
    
.Lloop:
    cmp r8, rsi
    jae .Ldone
    
    mov bl, BYTE PTR [rdi + r8]  # current char
    
    # Check if we're in quotes
    test r9, r9
    jnz .Lin_quotes
    
    # Not in quotes - check for quote start
    cmp bl, 0x22              # '"'
    je .Lstart_quote
    cmp bl, 0x27              # '\''
    je .Lstart_quote
    
    # Check for comma (split point)
    cmp bl, 0x2c              # ','
    je .Lfound_comma
    
    jmp .Lnext_char
    
.Lstart_quote:
    mov r9, 1                 # in_quotes = true
    mov r10, rbx              # quote_char = current char
    jmp .Lnext_char
    
.Lin_quotes:
    # Check if this char matches our quote char
    cmp bl, r10b
    jne .Lnext_char
    
    # End quote
    xor r9, r9                # in_quotes = false
    xor r10, r10              # quote_char = 0
    jmp .Lnext_char
    
.Lfound_comma:
    # Check if we have space in splits array
    cmp rax, rcx
    jae .Lnext_char           # No more space
    
    # Store the comma position
    mov QWORD PTR [rdx + rax*8], r8
    inc rax                   # split_count++
    
.Lnext_char:
    inc r8                    # i++
    jmp .Lloop
    
.Ldone:
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    pop rbp
    ret

.size split_quotes_scan, .-split_quotes_scan