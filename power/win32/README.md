## 執行

```
PS C:\Users\Owner\Desktop\110710520-1\sp108b\power\win32>  gcc .\JitCall.c -o power
PS C:\Users\Owner\Desktop\110710520-1\sp108b\power\win32> ./power
pow(5, 2)=25
```

## 原理

```
PS C:\Users\Owner\Desktop\110710520-1\sp108b\power\win32\c> gcc -c power.c
PS C:\Users\Owner\Desktop\110710520-1\sp108b\power\win32\c> objdump -d power.o

power.o:     file format pe-i386


Disassembly of section .text:   

00000000 <_power>:
   0:   55                      push   %ebp
   1:   89 e5                   mov    %esp,%ebp
   3:   83 ec 10                sub    $0x10,%esp
   6:   c7 45 fc 01 00 00 00    movl   $0x1,-0x4(%ebp)
   d:   8b 45 0c                mov    0xc(%ebp),%eax
  10:   89 45 f8                mov    %eax,-0x8(%ebp)
  13:   eb 0e                   jmp    23 <_power+0x23>
  15:   8b 45 fc                mov    -0x4(%ebp),%eax
  18:   0f af 45 08             imul   0x8(%ebp),%eax
  1c:   89 45 fc                mov    %eax,-0x4(%ebp)
  1f:   83 6d f8 01             subl   $0x1,-0x8(%ebp)
  23:   83 7d f8 00             cmpl   $0x0,-0x8(%ebp)
  27:   7f ec                   jg     15 <_power+0x15>
  29:   8b 45 fc                mov    -0x4(%ebp),%eax
  2c:   c9                      leave
  2d:   c3                      ret
  2e:   90                      nop
  2f:   90                      nop
```
