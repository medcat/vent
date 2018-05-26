.code16
.global _entry
.text

.macro oe9 value
    mov \value, %al
    outb %al, $0xe9
.endm
_entry:
    oe9 $'H
    oe9 $'e
    oe9 $'l
    oe9 $'l
    oe9 $'o
    oe9 $' 
    oe9 $'w
    oe9 $'o
    oe9 $'r
    oe9 $'l
    oe9 $'d
    hlt
_entry_end:
