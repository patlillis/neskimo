# Notes

Some general notes on the NES, including architecture and other tidbits. Heavily
borrowed from [Niels Widger's blog post](http://nwidger.github.io/blog/post/writing-an-nes-emulator-in-go-part-1/).

## CPU

*   MOS 6502 chip
*   Runs at 1.789773Mhz, or 1,789,773 cycles per second
*   Does not support decimal mode.
*   8-bit processor
*   16-bit addresses, little-endian (so least-significant bit is stored first
    in memory).
*   no I/O lines, so I/O registers must be mapped into the 16-bit address space.
*   More details on [Wikipedia](https://en.wikipedia.org/wiki/MOS_Technology_6502#Technical_description),
    [nesdev.com](http://wiki.nesdev.com/w/index.php/CPU_ALL), and [6502.org](http://www.6502.org/tutorials/6502opcodes.html).

### Registers

All registers are 8-bit, except for `PC` which is 16-bit.

*   **Accumulator** (`A`): The `A` register is used for all arithmetic and logic
    instructions.
*   **Index 1 & 2** (`X` and `Y`): Registers `X` and `Y` are used for indirect
    addressing and also as counters/indexes. `X` is used by certain instructions
    to save/restore the value of `P` using the stack.
*   **Stack Pointer** (`SP`): Stores the least-significant byte of the top of the
    stack. The 6052's stack is hardwired to occupy `$0100` - `$01ff` with `SP`
    initialized to `$ff` at power-up (stack is at `$01ff`). For example, if the
    value of `SP` is `$84` then the top of the stack is located at `$0184`. The
    top of the stack moves downward in memory as values are pushed and upwards
    as values are popped.
*   **Program Counter** (`PC`): The only 16-bit register on the 6502, `PC` points
    to the next instruction to execute.
*   **Processor Status** (`P`): The bits in `P` indicate the results of the last
    arithmetic and logic instructions as well as indicate if a break/interrupt
    instruction has just been executed.
    *   **Bit 0** (`C`): Carry Flag
    *   **Bit 1** (`Z`): Zero Flag
    *   **Bit 2** (`I`): Interrupt Disable
    *   **Bit 3** (`D`): Decimal Mode
    *   **Bit 4** (`B`): Break Command
    *   **Bit 5**: _UNUSED_
    *   **Bit 6** (`O`): Overflow Flag
    *   **Bit 7** (`N`): Negative Flag

### Memory Map

*   `$0000` - `$00ff`: Used by zero page addressing instructions. Instructions
    using zero page addressing only require an 8-bit address parameter. The
    most-significant 8 bits of the address are assumed to be `$00`. This is done
    to save memory since the address requires only half the space.
*   `$0100` - `$01ff`: Reserved for the system stack.
*   `$0200` - `$fff9`: UNSPECIFIED
*   `$fffa` - `$fffb`: Contains the address of non-maskable interrupt (NMI)
    handler.
*   `$fffc` - `$fffd`: Contains the address of reset location.
*   `$fffe` - `$ffff`: Contains the address of BRK/IRQ handler.

### Opcodes

See [6502.org](http://www.6502.org/tutorials/6502opcodes.html) for a list of all the Opcodes.

The NES also included some [unofficial opcodes](http://wiki.nesdev.com/w/index.php/Programming_with_unofficial_opcodes) that were officially discouraged, but still had specific functions and could be made useful for some games.

### Addressing Modes

The 6502 uses 16-bit addresses. Good overviews can be found at [skilldrick.github.io/easy6502](https://skilldrick.github.io/easy6502/#addressing) and [www.emulator101.com](http://www.emulator101.com.s3-website-us-east-1.amazonaws.com/6502-addressing-modes.html).

#### Absolute: `$c000`

With absolute addressing, the full memory location is included as an argument to the instruction. For example:

```
STA $c000 ;Store the value in the accumulator to memory location $c000
```

#### Zero page: `$c0`

Similar to absolute addressing, except only the lower byte is specified in the instruction; the high-order byte is `$00`. This means that only the first page (the first 256 bytes) of memory is accessible - hence the term "zero page" addressing. This is faster, as only one byte needs to be looked up, and takes up less space in the assembled code as well.

#### Zero page,X: `$c0,X`

Similar to zero page addressing, but the value of the `X` register is added to the address. Note that the target address will wrap around to always be in the zero page. For example:

```
LDX #$01  ;Load the value $01 into register X
STA $a0,X ;Store the value in the accumulator to memory location $00a1
```

#### Zero page,Y: `$c0,Y`

The equivalent of zero page,X addressing, but can only be used with `LDX` and `STX`.

#### Absolute,X and absolute,Y: `$c000,X` and `$c000,Y`

These are the absolute addressing versions of zero page,X and zero page,Y. For example:

```
LDX #$01    ;Load the value $01 into register X
STA $0200,X ;Store the value in the accumulator to memory location $0201
```

#### Immediate: `#$c0`

Immediate addressing doesn't strictly deal with memory addresses. Instead, actual values are specified directly
in the instruction. For example:

```
LDX #$01 ;Load the value $01 into register X
```

Contrast this with immediate addressing:

```
LDX $01 ;Load the value at memory location $01 into register X
```

#### Relative: `$c0` (or label)

Relative addressing is used for branching instructions. These instructions take a single byte, which is used as an offset from the following instruction. Note that this byte is signed, so it can jump a maximum of 127 bytes forward, or 128 bytes backward.

#### Implicit

Some instructions don't deal with memory locations (for example `INX` which just increments the `X` register). These are said to have implicit addressing; the argument is implied by the instruction.

#### Indirect: `($c000)`

Indirect addressing uses an absolute address to look up another address. The address specified in the instruction
is the location of where to look up the least significant byte of the address. The most significant byte is at the following location in memory.

#### Indexed indirect: `($c0,X)`

Sort of a cross between zero page,X and indirect. Instead of looking up the address at the location specified (as in indirect mode), you take the zero page address, add the value of the `X` register to it, then use that to look up a two-byte address. Note that the `X` register is added to the zero page address _before_ dereferencing.

#### Indirect indexed: `($c0),Y`

Similar to Indexed indirect, except instead of adding the `X` register to the address _before_ the zero page address is dereferenced, the zero page address is first dereferenced, and _then_ the `Y` register is added to the resulting address.

#### Implied

For some instructions, the data and/or destination address is implied in the instruction itself, and does not need to be explicitly stated. For example, the `CLC` instruction is implied, it is going to clear the processor's Carry flag.

### Fetch/Execute Cycle

*   Each cycle, CPU fetches instruction at the address stored in `PC`, looks up
    the opcode in the instruction table, and then executes it.
*   At the end of each cycle, `PC` should increment.
*   Each instruction needs to determine how many clock cycles it should use up.

### Clock

*   The 6502 has specific timings in order for the CPU to interact with other
    components of the NES such as the PPU and API.
*   CPU clock needs to stay in sync with master clock.
*   Different instructions can take different amoutns of clock cycles.
*   Modern machine will almost certainly be executing emulator cycles much
    faster than a real 6502 chip, so some throttling will be needed.

### Interrupts

This is a way for the microprocessor to set the program counter, independent of instructions under control of the programmer, i.e. based on external signals. Certain locations in memory are reserved as "Vector pointers", which store a 16 bit address. On an external signal (e.g. Reset), the value in this vector pointer is loaded into the program counter. These Vector pointers are controllable by a program, and thus provides the means to react to different external signals.

These different signals are:

* NMI (Non-maskable Interrupt)
* RESET
* IRQ (Interrupt Request)
* BRK (Break)

These Vector pointers are located as follows:


|SIGNAL  | VECTOR          |
|--------|-----------------|
|NMI	 | `$FFFA`/`$FFFB` |
|RESET	 | `$FFFC`/`$FFFD` |
|IRQ/BRK | `$FFFE`/`$FFFF` |


## APU

The NES has an audio processing unit for generating sound in games. It is implemented in the RP2A03 (NTSC) and RP2A07 (PAL) chips. A good overview can be found at [nesdev.com](http://wiki.nesdev.com/w/index.php/APU).

### Channels

The APU has five channels: two pulse waves, triangle wave, noise, and DMC (sampling).

The channel registers begin at `$4000`, and each channel has four registers devoted to it. All but the triangle wave have 4-bit volume control (the triangle just has a mute/unmute flag).

*   `$4000` - `$4003`: Pulse wave 1
*   `$4004` - `$4007`: Pulse wave 2
*   `$4008` - `$400b`: Triangle wave
*   `$400c` - `$400f`: Noise
