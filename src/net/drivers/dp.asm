COMMAND equ 300h
PAGESTART equ COMMAND+1
PAGESTOP equ COMMAND+2
BOUNDARY equ COMMAND+3
TRANSMITSTATUS equ COMMAND+4
TRANSMITPAGE equ COMMAND+4
TRANSMITBYTECOUNT0 equ COMMAND+5
NCR equ COMMAND+5
TRANSMITBYTECOUNT1 equ COMMAND+6
INTERRUPTSTATUS equ COMMAND+7
CURRENT equ COMMAND+7 ;in page 1
REMOTESTARTADDRESS0 equ COMMAND+8
CRDMA0 equ COMMAND+8
REMOTESTARTADDRESS1 equ COMMAND+9
CRDMA1 equ COMMAND+9
REMOTEBYTECOUNT0 equ COMMAND+0ah
REMOTEBYTECOUNT1 equ COMMAND+0bh
RECEIVESTATUS equ COMMAND+0ch
RECEIVECONFIGURATION equ COMMAND+0ch
TRANSMITCONFIGURATION equ COMMAND+0dh
FAE_TALLY equ COMMAND+0dh
DATACONFIGURATION equ COMMAND+0eh
CRC_TALLY equ COMMAND+0eh
INTERRUPTMASK equ COMMAND+0fh
MISS_PKT_TALLY equ COMMAND+0fh

PSTART equ 46h
PSTOP equ 80h
