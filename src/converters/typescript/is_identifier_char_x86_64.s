.text
.globl ts_is_identifier_char
.type ts_is_identifier_char, @function
# uint8_t ts_is_identifier_char(uint8_t b)
ts_is_identifier_char:
    # Argument in dil (lower 8 bits of rdi)
    movzx eax, dil        # zero-extend to eax

    # Check '_' (0x5F) or '$' (0x24)
    cmp al, 0x5F
    je .Lyes
    cmp al, 0x24
    je .Lyes

    # Check '0'..'9'
    cmp al, '0'
    jb .Lno
    cmp al, '9'
    jbe .Lyes

    # Check 'A'..'Z'
    cmp al, 'A'
    jb .Lno
    cmp al, 'Z'
    jbe .Lyes

    # Check 'a'..'z'
    cmp al, 'a'
    jb .Lno
    cmp al, 'z'
    jbe .Lyes

.Lno:
    xor eax, eax          # return 0
    ret

.Lyes:
    mov eax, 1            # return 1
    ret
.size ts_is_identifier_char, .-ts_is_identifier_char 