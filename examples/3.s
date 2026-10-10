	.globl main
main:
	pushq	%rbp
	movq	%rsp, %rbp
	movq	$2, %rax
	pushq	%rax
	movq	$7, %rax
	movq	%rax, %rcx
	popq	%rax
	imulq	%rcx
	movq	%rbp, %rsp
	popq	%rbp
	ret
