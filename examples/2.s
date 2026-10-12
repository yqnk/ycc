	.globl main
main:
	pushq	%rbp
	movq	%rsp, %rbp
	movq	$3, %rax
	cmpq	$0, %rax
	sete	%al
	movzbl	%al, %eax
	notq	%rax
	negq	%rax
	cmpq	$0, %rax
	sete	%al
	movzbl	%al, %eax
	cmpq	$0, %rax
	sete	%al
	movzbl	%al, %eax
	pushq	%rax
	movq	$3, %rax
	movq	%rax, %rcx
	popq	%rax
	addq	%rcx, %rax
	movq	%rbp, %rsp
	popq	%rbp
	ret
