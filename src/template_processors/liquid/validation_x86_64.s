.text
.globl liquid_is_valid_variable_name
.type liquid_is_valid_variable_name, @function
# uint8_t liquid_is_valid_variable_name(const uint8_t* ptr, size_t len)
# Rules (ASCII fast path only):
#   - empty -> invalid
#   - first: [A-Za-z_] only
#   - rest:  [A-Za-z0-9_.]
liquid_is_valid_variable_name:
	# rdi=ptr, rsi=len
	# if len == 0 -> invalid
	test rsi, rsi
	jz .Lvalidation_invalid

	# Load first byte
	movzx eax, BYTE PTR [rdi]

	# Check underscore '_'
	cmp al, '_'
	je .Lvalidation_first_ok

	# Check 'A'..'Z'
	cmp al, 'A'
	jb .Lvalidation_invalid
	cmp al, 'Z'
	jbe .Lvalidation_first_ok

	# Check 'a'..'z'
	cmp al, 'a'
	jb .Lvalidation_invalid
	cmp al, 'z'
	jbe .Lvalidation_first_ok

	# Not in allowed set for first char
	jmp .Lvalidation_invalid

.Lvalidation_first_ok:
	# Advance
	inc rdi
	dec rsi
	jz .Lvalidation_valid

	# Loop over remaining bytes
.Lvalidation_loop:
	movzx eax, BYTE PTR [rdi]

	# Check '0'..'9'
	cmp al, '0'
	jb .Lvalidation_check_alpha
	cmp al, '9'
	jbe .Lvalidation_char_ok

.Lvalidation_check_alpha:
	# Check 'A'..'Z'
	cmp al, 'A'
	jb .Lvalidation_check_dot_underscore
	cmp al, 'Z'
	jbe .Lvalidation_char_ok

	# Check 'a'..'z'
	cmp al, 'a'
	jb .Lvalidation_check_dot_underscore
	cmp al, 'z'
	jbe .Lvalidation_char_ok

.Lvalidation_check_dot_underscore:
	cmp al, '.'
	je .Lvalidation_char_ok
	cmp al, '_'
	je .Lvalidation_char_ok
	jmp .Lvalidation_invalid

.Lvalidation_char_ok:
	inc rdi
	dec rsi
	jnz .Lvalidation_loop

.Lvalidation_valid:
	mov eax, 1
	ret

.Lvalidation_invalid:
	xor eax, eax
	ret
.size liquid_is_valid_variable_name, .-liquid_is_valid_variable_name 