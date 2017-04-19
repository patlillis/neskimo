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
    [nesdev.com](http://nesdev.com/6502.txt), and [6502.org](http://www.6502.org/tutorials/6502opcodes.html).

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
