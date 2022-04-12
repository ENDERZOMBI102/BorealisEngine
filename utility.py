from os import system
from pathlib import Path


clear = lambda: system('clear')
stack = []
pos = 0
vec = Path('filesystem_unc.upkf').read_bytes()


u8 = 1
u16 = 2
u32 = 4
u64 = 8
u128 = 16


def read(amount: int) -> bytes:
    global pos, stack
    stack += [ amount ]
    return vec[ pos : ( pos := pos + amount ) ]


def undo() -> None:
    global pos, stack
    pos -= stack.pop()
