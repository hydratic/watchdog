entering_v86:
	mov epb, esp               ; save stack pointer

  	push dword  [ebp+4]        ; ss
		push dword  [ebp+8]        ; esp
  	pushfd                     ; eflags
  	or dword [esp], (1 << 17)  ; set VM flags
  	push dword [ebp+12]        ; cs
  	push dword  [ebp+16]       ; eip
  	iret
