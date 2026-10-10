	.globl main
main:
	movq	$2, %rax
	pushq	%rax
	movq	$7, %rax
	movq	%rax, %rcx
	popq	%rax
	imulq	%rcx
	ret
