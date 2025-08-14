.text
.globl find_equal_scan
.type find_equal_scan, @function
# size_t find_equal_scan(const uint8_t* ptr, size_t len) // returns index or usize::MAX
find_equal_scan:
	# Args: rdi=ptr, rsi=len
	xor rax, rax              # i = 0
	test rsi, rsi
	je .Lfind_equal_not_found
.Lfind_equal_loop:
	mov bl, BYTE PTR [rdi + rax]
	cmp bl, 0x3d            # '='
	je .Lfind_equal_found
	inc rax
	cmp rax, rsi
	jb .Lfind_equal_loop
.Lfind_equal_not_found:
	mov rax, -1             # usize::MAX
	ret
.Lfind_equal_found:
	ret
.size find_equal_scan, .-find_equal_scan 