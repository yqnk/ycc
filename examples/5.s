	.globl main
main:
	pushq	%rbp
	movq	%rsp, %rbp
	movq	$42, %rax
	pushq	%rax
	movq	$5, %rax
	movq	%rax, %rcx
	movq	-8(%rbp), %rax
	addq	%rcx, %rax
	movq	%rax, -8(%rbp)
	movq	$0, %rax
	movq	%rbp, %rsp
	popq	%rbp
	ret
