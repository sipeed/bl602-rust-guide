set architecture riscv:rv32

target extended-remote :3333

# print demangled symbols
set print asm-demangle on

# set backtrace limit to not have infinite backtrace loops
set backtrace limit 32

mem 0x22008000 0x22014000 rw
mem 0x42008000 0x42014000 rw
mem 0x22014000 0x22020000 rw
mem 0x42014000 0x42020000 rw
mem 0x22020000 0x22030000 rw
mem 0x42020000 0x42030000 rw
mem 0x22030000 0x2204C000 rw
mem 0x42030000 0x4204C000 rw
mem 0x23000000 0x23400000 ro

load

break _start

# start the process but immediately halt the processor
stepi
