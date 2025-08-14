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
    je .Lsplit_done
    
.Lsplit_loop:
    cmp r8, rsi
    jae .Lsplit_done
    
    mov bl, BYTE PTR [rdi + r8]  # current char
    
    # Check if we're in quotes
    test r9, r9
    jnz .Lsplit_in_quotes
    
    # Not in quotes - check for quote start
    cmp bl, 0x22              # '"'
    je .Lsplit_start_quote
    cmp bl, 0x27              # '\''
    je .Lsplit_start_quote
    
    # Check for comma (split point)
    cmp bl, 0x2c              # ','
    je .Lsplit_found_comma
    
    jmp .Lsplit_next_char
    
.Lsplit_start_quote:
    mov r9, 1                 # in_quotes = true
    mov r10, rbx              # quote_char = current char
    jmp .Lsplit_next_char
    
.Lsplit_in_quotes:
    # Check if this char matches our quote char
    cmp bl, r10b
    jne .Lsplit_next_char
    
    # End quote
    xor r9, r9                # in_quotes = false
    xor r10, r10              # quote_char = 0
    jmp .Lsplit_next_char
    
.Lsplit_found_comma:
    # Check if we have space in splits array
    cmp rax, rcx
    jae .Lsplit_next_char     # No more space
    
    # Store the comma position
    mov QWORD PTR [rdx + rax*8], r8
    inc rax                   # split_count++
    
.Lsplit_next_char:
    inc r8                    # i++
    jmp .Lsplit_loop
    
.Lsplit_done:
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    pop rbp
    ret

.size split_quotes_scan, .-split_quotes_scan