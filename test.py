from dataclasses import dataclass
from sys import argv as args
from typing import Optional, Literal


@dataclass
class Arg:
	type: Literal['arg', 'value']
	value: str


@dataclass
class Argument:
	key: str
	value: Optional[str]


executable: Optional[str] = None
raw_arguments: list[Arg] = []
arguments: list[Argument] = []


for arg in args:
	if executable is None:
		executable = arg
	else:
		raw_arguments.append( Arg('arg', arg) if arg.startswith( ('-', '+') ) else Arg('value', arg) )

i = 0
while i < len( raw_arguments ):
	if len(raw_arguments) > i + 1 and raw_arguments[i + 1].type == 'value':
		arguments.append(Argument(raw_arguments[i].value, raw_arguments[i + 1].value))
		i += 2
	else:
		arguments.append( Argument(raw_arguments[i].value, None) )
		i += 1


print(f"Executable:\n\t- \"{executable}\"")
print("Arguments:")
for arg in arguments:
	print(f"\t- \"{arg}\"")