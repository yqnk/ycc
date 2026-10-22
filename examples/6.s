	.globl main
main:
	pushq	%rbp
	movq	%rsp, %rbp
	movq	$2, %rax
	pushq	%rax
	movq	$6, %rax
	movq	%rax, %rcx
	popq	%rax
	cmpq	%rcx, %rax
	sete	%al
	movzbl	%al, %eax
	cmpq	$0, %rax
	je	.L_else_0
	movq	$9, %rax
	pushq	%rax
	addq	$8, %rsp
	jmp	.L_post_0
.L_else_0:
.L_post_0:
	movq	$0, %rax
	movq	%rbp, %rsp
	popq	%rbp
	ret
