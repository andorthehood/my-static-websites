.text
.globl single_line_breaks_scan
.type single_line_breaks_scan, @function
# size_t single_line_breaks_scan(const uint8_t* input, size_t input_len, uint8_t* output, size_t output_capacity)
# Scans input string and replaces '\n' with "<br />" in the output buffer
# Returns the actual length of output written (or SIZE_MAX if output_capacity is too small)
single_line_breaks_scan:
    push rbp
    mov rbp, rsp
    push r12
    push r13
    push r14
    push r15
    
    # Arguments: rdi=input, rsi=input_len, rdx=output, rcx=output_capacity
    xor rax, rax              # input_pos = 0
    xor r8, r8                # output_pos = 0
    
    test rsi, rsi
    jz .Ldone_ok              # empty input case
    
.Lloop:
    cmp rax, rsi
    jae .Ldone_ok             # reached end of input
    
    # Load current input byte
    mov bl, BYTE PTR [rdi + rax]
    
    cmp bl, 0x0A              # compare with '\n' (0x0A)
    je .Lreplace_newline
    
    # Regular character - just copy
    cmp r8, rcx
    jae .Lbuffer_too_small    # check output buffer capacity
    mov BYTE PTR [rdx + r8], bl
    inc r8
    inc rax
    jmp .Lloop
    
.Lreplace_newline:
    # Need to write "<br />" (6 characters)
    mov r9, 6
    add r9, r8                # new output position after writing "<br />"
    cmp r9, rcx
    ja .Lbuffer_too_small
    
    # Write "<br />" to output
    mov BYTE PTR [rdx + r8], 0x3C      # '<'
    mov BYTE PTR [rdx + r8 + 1], 0x62  # 'b'
    mov BYTE PTR [rdx + r8 + 2], 0x72  # 'r'
    mov BYTE PTR [rdx + r8 + 3], 0x20  # ' '
    mov BYTE PTR [rdx + r8 + 4], 0x2F  # '/'
    mov BYTE PTR [rdx + r8 + 5], 0x3E  # '>'
    
    add r8, 6                 # advance output position
    inc rax                   # advance input position
    jmp .Lloop
    
.Lbuffer_too_small:
    mov rax, -1               # return SIZE_MAX to indicate buffer too small
    jmp .Lcleanup
    
.Ldone_ok:
    mov rax, r8               # return actual output length
    
.Lcleanup:
    pop r15
    pop r14
    pop r13
    pop r12
    mov rsp, rbp
    pop rbp
    ret

.size single_line_breaks_scan, .-single_line_breaks_scan

.globl count_newlines
.type count_newlines, @function
# size_t count_newlines(const uint8_t* input, size_t input_len)
# Counts the number of newline characters in the input
count_newlines:
    # Arguments: rdi=input, rsi=input_len
    xor rax, rax              # count = 0
    xor rcx, rcx              # i = 0
    
    test rsi, rsi
    jz .Lcount_done           # empty input
    
.Lcount_loop:
    cmp rcx, rsi
    jae .Lcount_done
    
    mov bl, BYTE PTR [rdi + rcx]
    cmp bl, 0x0A              # '\n'
    jne .Lcount_next
    inc rax                   # increment count
    
.Lcount_next:
    inc rcx
    jmp .Lcount_loop
    
.Lcount_done:
    ret

.size count_newlines, .-count_newlines