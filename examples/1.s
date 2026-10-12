	.globl main
main:
	pushq	%rbp
	movq	%rsp, %rbp
	movq	$67, %rax
	movq	%rbp, %rsp
	popq	%rbp
	ret
