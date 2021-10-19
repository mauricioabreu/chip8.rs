# chip8.rs

> chip-8 written in rust lang

## Usage

```
cargo run <rom_file>
```

There are some ROMs I used to test the project.

[test_opcode.ch8](https://github.com/corax89/chip8-test-rom)

```
cargo run roms/test_opcode.ch8
```

Output should be similar to

<img src="img/test_opcode.png" widh="512" height="256">

OpCodes tested with this ROM:

```
3XNN	00EE	8XY5
4XNN	8XY0	8XY6
5XY0	8XY1	8XYE
7XNN	8XY2	FX55
9XY0	8XY3	FX33
ANNN	8XY4	1NNN
```

[BC_test.ch8](https://github.com/daniel5151/AC8E/blob/master/roms/bc_test.txt)

This ROM tests conditional jumps, the mathematical and logical operations of chip-8

<img src="img/BC_test.png" widh="512" height="256">

Thanks to BestCoder (bestcoder@ymail.com) for the `BC_test.ch8` ROM
## Keypad

The keypad used has 16 keys. There is more than one layout out there but this emulator uses the following:

|   |   |   |   |
|---|---|---|---|
| 1 | 2 | 3 | 4 |
| Q | W | E | R |
| A | S | D | F |
| Z | X | C | V |

## Playing other ROMs

You can find a plenty of ROMs [here](https://www.zophar.net/pdroms/chip8/chip-8-games-pack.html)

They are public domain. You can download and use this emulator to play chip-8 games

<img src="img/invaders.png" widh="512" height="256">