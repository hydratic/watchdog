detect_v86:
   smsw    ax
   and     eax,1           ;CR0.PE bit
   ret
