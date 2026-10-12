	.globl main
main:
	pushq	%rbp
	movq	%rsp, %rbp
	movq	$67, %rax
	pushq	%rax
	movq	$67, %rax
	pushq	%rax
	movq	-8(%rbp), %rax
	movq	%rax, %rcx
	popq	%rax
	addq	%rcx, %rax
	movq	%rbp, %rsp
	popq	%rbp
	ret
