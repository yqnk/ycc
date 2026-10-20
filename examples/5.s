	.globl main
main:
	pushq	%rbp
	movq	%rsp, %rbp
	movq	$8, %rax
	pushq	%rax
	movq	$0, %rax
	pushq	%rax
	movq	-8(%rbp), %rax
	pushq	%rax
	movq	$9, %rax
	movq	%rax, %rcx
	popq	%rax
	cmpq	%rcx, %rax
	sete	%al
	movzbl	%al, %eax
	cmpq	$0, %rax
	je	.L_else_0
	movq	$16, %rax
	movq	%rax, -16(%rbp)
	jmp	.L_post_0
.L_else_0:
	movq	$67, %rax
	movq	%rax, -16(%rbp)
.L_post_0:
	movq	-16(%rbp), %rax
	movq	%rbp, %rsp
	popq	%rbp
	ret
