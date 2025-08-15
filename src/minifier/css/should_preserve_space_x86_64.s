.text
.globl should_preserve_space_scan
.type should_preserve_space_scan, @function
# uint8_t should_preserve_space_scan(uint8_t last_char, uint8_t next_char) // returns 1 if space should be preserved, 0 if not
should_preserve_space_scan:
    # Args: rdi=last_char (only low 8 bits used), rsi=next_char (only low 8 bits used)
    mov al, dil               # move last_char to al
    mov bl, sil               # move next_char to bl
    
    # Check if last_char is null (empty string case)
    test al, al
    jz .Lno_preserve_space
    
    # Check for: (last_char.is_ascii_digit() || last_char == '%') && next_char.is_alphabetic()
    # First check if last_char is digit (0-9) or '%'
    cmp al, 48                # '0'
    jb .Lcheck_percent
    cmp al, 57                # '9'
    jbe .Lcheck_next_alpha    # last_char is digit, check if next_char is alphabetic
    
.Lcheck_percent:
    cmp al, 37                # '%'
    je .Lcheck_next_alpha
    jmp .Lcheck_percent_digit
    
.Lcheck_next_alpha:
    # Check if next_char is alphabetic (a-z or A-Z)
    cmp bl, 65                # 'A'
    jb .Lcheck_percent_digit
    cmp bl, 90                # 'Z'
    jbe .Lpreserve_space      # A-Z range
    cmp bl, 97                # 'a'
    jb .Lcheck_percent_digit
    cmp bl, 122               # 'z'
    jbe .Lpreserve_space      # a-z range
    
.Lcheck_percent_digit:
    # Check for: last_char == '%' && next_char.is_ascii_digit()
    cmp al, 37                # '%'
    jne .Lcheck_alpha_digit_hash
    cmp bl, 48                # '0'
    jb .Lcheck_alpha_digit_hash
    cmp bl, 57                # '9'
    jbe .Lpreserve_space      # next_char is digit
    
.Lcheck_alpha_digit_hash:
    # Check for: last_char.is_alphabetic() && (next_char.is_ascii_digit() || next_char == '#')
    # First check if last_char is alphabetic
    cmp al, 65                # 'A'
    jb .Lcheck_units_alpha
    cmp al, 90                # 'Z'
    jbe .Lcheck_next_digit_or_hash  # A-Z range
    cmp al, 97                # 'a'
    jb .Lcheck_units_alpha
    cmp al, 122               # 'z'
    ja .Lcheck_units_alpha    # not in a-z range
    
.Lcheck_next_digit_or_hash:
    # Check if next_char is digit or '#'
    cmp bl, 48                # '0'
    jb .Lcheck_next_hash
    cmp bl, 57                # '9'
    jbe .Lpreserve_space      # next_char is digit
    
.Lcheck_next_hash:
    cmp bl, 35                # '#'
    je .Lpreserve_space
    
.Lcheck_units_alpha:
    # Check for: (last_char == 'x' || last_char == 'm' || last_char == '%') && next_char.is_alphabetic()
    cmp al, 120               # 'x'
    je .Lcheck_units_next_alpha
    cmp al, 109               # 'm'
    je .Lcheck_units_next_alpha
    cmp al, 37                # '%'
    je .Lcheck_units_next_alpha
    jmp .Lcheck_paren
    
.Lcheck_units_next_alpha:
    # Check if next_char is alphabetic
    cmp bl, 65                # 'A'
    jb .Lcheck_paren
    cmp bl, 90                # 'Z'
    jbe .Lpreserve_space      # A-Z range
    cmp bl, 97                # 'a'
    jb .Lcheck_paren
    cmp bl, 122               # 'z'
    jbe .Lpreserve_space      # a-z range
    
.Lcheck_paren:
    # Check for: last_char == ')' && (next_char.is_ascii_digit() || next_char.is_alphabetic())
    cmp al, 41                # ')'
    jne .Lcheck_comma_hash
    # Check if next_char is digit
    cmp bl, 48                # '0'
    jb .Lcheck_paren_alpha
    cmp bl, 57                # '9'
    jbe .Lpreserve_space      # next_char is digit
    
.Lcheck_paren_alpha:
    # Check if next_char is alphabetic
    cmp bl, 65                # 'A'
    jb .Lcheck_comma_hash
    cmp bl, 90                # 'Z'
    jbe .Lpreserve_space      # A-Z range
    cmp bl, 97                # 'a'
    jb .Lcheck_comma_hash
    cmp bl, 122               # 'z'
    jbe .Lpreserve_space      # a-z range
    
.Lcheck_comma_hash:
    # Check for: last_char == ',' && next_char == '#'
    cmp al, 44                # ','
    jne .Lcheck_digit_hash
    cmp bl, 35                # '#'
    je .Lpreserve_space
    
.Lcheck_digit_hash:
    # Check for: last_char.is_ascii_digit() && next_char == '#'
    cmp al, 48                # '0'
    jb .Lcheck_alnum_dot
    cmp al, 57                # '9'
    ja .Lcheck_alnum_dot
    cmp bl, 35                # '#'
    je .Lpreserve_space
    
.Lcheck_alnum_dot:
    # Check for: (last_char.is_alphanumeric() || last_char == ']' || last_char == ')') && next_char == '.'
    # First check if next_char is '.'
    cmp bl, 46                # '.'
    jne .Lcheck_alpha_hash
    
    # Check if last_char is alphanumeric
    cmp al, 48                # '0'
    jb .Lcheck_bracket_paren_dot
    cmp al, 57                # '9'
    jbe .Lpreserve_space      # digit
    cmp al, 65                # 'A'
    jb .Lcheck_bracket_paren_dot
    cmp al, 90                # 'Z'
    jbe .Lpreserve_space      # uppercase letter
    cmp al, 97                # 'a'
    jb .Lcheck_bracket_paren_dot
    cmp al, 122               # 'z'
    jbe .Lpreserve_space      # lowercase letter
    
.Lcheck_bracket_paren_dot:
    # Check for ']' or ')'
    cmp al, 93                # ']'
    je .Lpreserve_space
    cmp al, 41                # ')'
    je .Lpreserve_space
    
.Lcheck_alpha_hash:
    # Check for: last_char.is_alphabetic() && next_char == '#'
    cmp bl, 35                # '#'
    jne .Lcheck_before_minus
    cmp al, 65                # 'A'
    jb .Lcheck_before_minus
    cmp al, 90                # 'Z'
    jbe .Lpreserve_space      # A-Z range
    cmp al, 97                # 'a'
    jb .Lcheck_before_minus
    cmp al, 122               # 'z'
    jbe .Lpreserve_space      # a-z range
    
.Lcheck_before_minus:
    # Check for: (various conditions) && next_char == '-'
    cmp bl, 45                # '-'
    jne .Lcheck_alnum_general
    
    # Check if last_char is digit
    cmp al, 48                # '0'
    jb .Lcheck_minus_units
    cmp al, 57                # '9'
    jbe .Lpreserve_space      # digit
    
.Lcheck_minus_units:
    # Check for 'm', 'x', '%'
    cmp al, 109               # 'm'
    je .Lpreserve_space
    cmp al, 120               # 'x'
    je .Lpreserve_space
    cmp al, 37                # '%'
    je .Lpreserve_space
    
    # Check if last_char is alphabetic (for "word and negative numbers")
    cmp al, 65                # 'A'
    jb .Lcheck_alnum_general
    cmp al, 90                # 'Z'
    jbe .Lpreserve_space      # A-Z range
    cmp al, 97                # 'a'
    jb .Lcheck_alnum_general
    cmp al, 122               # 'z'
    jbe .Lpreserve_space      # a-z range
    
.Lcheck_alnum_general:
    # Check for: last_char.is_alphanumeric() && next_char.is_alphanumeric()
    # First check if last_char is alphanumeric
    cmp al, 48                # '0'
    jb .Lno_preserve_space
    cmp al, 57                # '9'
    jbe .Lcheck_next_alnum    # digit
    cmp al, 65                # 'A'
    jb .Lno_preserve_space
    cmp al, 90                # 'Z'
    jbe .Lcheck_next_alnum    # uppercase letter
    cmp al, 97                # 'a'
    jb .Lno_preserve_space
    cmp al, 122               # 'z'
    ja .Lno_preserve_space    # not lowercase letter
    
.Lcheck_next_alnum:
    # Check if next_char is alphanumeric
    cmp bl, 48                # '0'
    jb .Lno_preserve_space
    cmp bl, 57                # '9'
    jbe .Lpreserve_space      # digit
    cmp bl, 65                # 'A'
    jb .Lno_preserve_space
    cmp bl, 90                # 'Z'
    jbe .Lpreserve_space      # uppercase letter
    cmp bl, 97                # 'a'
    jb .Lno_preserve_space
    cmp bl, 122               # 'z'
    jbe .Lpreserve_space      # lowercase letter
    
.Lno_preserve_space:
    xor rax, rax              # return 0 (don't preserve space)
    ret

.Lpreserve_space:
    mov rax, 1                # return 1 (preserve space)
    ret
    
.size should_preserve_space_scan, .-should_preserve_space_scan