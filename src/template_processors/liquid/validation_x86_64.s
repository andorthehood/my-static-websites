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
	jz .Linvalid

	# Load first byte
	movzx eax, BYTE PTR [rdi]

	# Check underscore '_'
	cmp al, '_'
	je .Lfirst_ok

	# Check 'A'..'Z'
	cmp al, 'A'
	jb .Linvalid
	cmp al, 'Z'
	jbe .Lfirst_ok

	# Check 'a'..'z'
	cmp al, 'a'
	jb .Linvalid
	cmp al, 'z'
	jbe .Lfirst_ok

	# Not in allowed set for first char
	jmp .Linvalid

.Lfirst_ok:
	# Advance
	inc rdi
	dec rsi
	jz .Lvalid

	# Loop over remaining bytes
.Lloop:
	movzx eax, BYTE PTR [rdi]

	# Check '0'..'9'
	cmp al, '0'
	jb .Lcheck_alpha
	cmp al, '9'
	jbe .Lchar_ok

.Lcheck_alpha:
	# Check 'A'..'Z'
	cmp al, 'A'
	jb .Lcheck_dot_underscore
	cmp al, 'Z'
	jbe .Lchar_ok

	# Check 'a'..'z'
	cmp al, 'a'
	jb .Lcheck_dot_underscore
	cmp al, 'z'
	jbe .Lchar_ok

.Lcheck_dot_underscore:
	cmp al, '.'
	je .Lchar_ok
	cmp al, '_'
	je .Lchar_ok
	jmp .Linvalid

.Lchar_ok:
	inc rdi
	dec rsi
	jnz .Lloop

.Lvalid:
	mov eax, 1
	ret

.Linvalid:
	xor eax, eax
	ret
.size liquid_is_valid_variable_name, .-liquid_is_valid_variable_name 