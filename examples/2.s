	.globl main
main:
	movq	$3, %rax
	cmpq	$0, %rax
	movq	$0, %rax
	sete	%al
	notq	%rax
	negq	%rax
	cmpq	$0, %rax
	movq	$0, %rax
	sete	%al
	cmpq	$0, %rax
	movq	$0, %rax
	sete	%al
	pushq	%rax
	movq	$3, %rax
	movq	%rax, %rcx
	popq	%rax
	add	%rcx, %rax
	ret
