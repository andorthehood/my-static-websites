.text
.globl find_equal_scan
.type find_equal_scan, @function
# size_t find_equal_scan(const uint8_t* ptr, size_t len) // returns index or usize::MAX
find_equal_scan:
	# Args: rdi=ptr, rsi=len
	xor rax, rax              # i = 0
	test rsi, rsi
	je .Lnot_found
.Lloop:
	mov bl, BYTE PTR [rdi + rax]
	cmp bl, 0x3d            # '='
	je .Lfound
	inc rax
	cmp rax, rsi
	jb .Lloop
.Lnot_found:
	mov rax, -1             # usize::MAX
	ret
.Lfound:
	ret
.size find_equal_scan, .-find_equal_scan 