	.globl main
main:
	pushq	%rbp
	movq	%rsp, %rbp
	movq	$2, %rax
	cmpq	$0, %rax
	sete	%al
	movzbl	%al, %eax
	cmpq	$0, %rax
	sete	%al
	movzbl	%al, %eax
	notq	%rax
	notq	%rax
	negq	%rax
	pushq	%rax
	movq	$7, %rax
	movq	%rax, %rcx
	popq	%rax
	imulq	%rcx
	pushq	%rax
	movq	$0, %rax
	notq	%rax
	cmpq	$0, %rax
	sete	%al
	movzbl	%al, %eax
	cmpq	$0, %rax
	sete	%al
	movzbl	%al, %eax
	movq	%rax, %rcx
	popq	%rax
	xorq	%rcx, %rax
	movq	%rbp, %rsp
	popq	%rbp
	ret
