Some general notes on the NES, including architecture and other tidbits. Heavily
borrowed from [Niels Widger's blog post](http://nwidger.github.io/blog/post/writing-an-nes-emulator-in-go-part-1/).

<!-- TOC -->

- [CPU](#cpu)
    - [Registers](#registers)
    - [Memory Map](#memory-map)
    - [Opcodes](#opcodes)
    - [Addressing Modes](#addressing-modes)
        - [Absolute: `$c000`](#absolute-c000)
        - [Zero page: `$c0`](#zero-page-c0)
        - [Zero page,X: `$c0,X`](#zero-pagex-c0x)
        - [Zero page,Y: `$c0,Y`](#zero-pagey-c0y)
        - [Absolute,X and absolute,Y: `$c000,X` and `$c000,Y`](#absolutex-and-absolutey-c000x-and-c000y)
        - [Immediate: `#$c0`](#immediate-c0)
        - [Relative: `$c0` (or label)](#relative-c0-or-label)
        - [Implicit](#implicit)
        - [Indirect: `($c000)`](#indirect-c000)
        - [Indexed indirect: `($c0,X)`](#indexed-indirect-c0x)
        - [Indirect indexed: `($c0),Y`](#indirect-indexed-c0y)
        - [Implied](#implied)
    - [Fetch/Execute Cycle](#fetchexecute-cycle)
    - [Clock](#clock)
    - [Interrupts](#interrupts)
    - [Interrupt Handling](#interrupt-handling)
        - [Resources](#resources)
- [PPU](#ppu)
    - [Registers](#registers-1)
        - [PPPUCTRL (`$2000`)](#pppuctrl-2000)
        - [PPUMASK (`$2001`)](#ppumask-2001)
        - [PPUSTATUS (`$2002`)](#ppustatus-2002)
        - [OAMADDR (`$2003`)](#oamaddr-2003)
            - [OAMDATA (`$2004`)](#oamdata-2004)
        - [PPUSCROLL (`$2005`)](#ppuscroll-2005)
        - [PPUADDR (`$2006`)](#ppuaddr-2006)
        - [PPUDATA (`$2007`)](#ppudata-2007)
        - [OAMDMA (`$4014`)](#oamdma-4014)
    - [Memory Map](#memory-map-1)
        - [Hardware mapping](#hardware-mapping)
    - [Rendering](#rendering)
    - [Scrolling](#scrolling)
- [APU](#apu)
    - [Channels](#channels)

<!-- /TOC -->

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

| Address Range     | Size    | Description                                                        |
| ----------------- | ------- | ------------------------------------------------------------------ |
| `$0000` - `$00ff` | `$0100` | Game RAM Used for zero page addressing instructions                |
| `$0100` - `$01ff` | `$0100` | Reserved for the system stack                                      |
| `$0200` - `$07ff` | `$0600` | RAM                                                                |
| `$0800` - `$0fff` | `$0800` | Mirror of `$0000` - `$07ff`                                        |
| `$1000` - `$17ff` | `$0800` | Mirror of `$0000` - `$07ff`                                        |
| `$1800` - `$1fff` | `$0800` | Mirror of `$0000` - `$07ff`                                        |
| `$2000` - `$2007` | `$0008` | PPU Registers                                                      |
| `$2008` - `$3fff` | `$1ff8` | Mirror of `$2000` - `$2007` (multple times)                        |
| `$4000` - `$401f` | `$0020` | APU Registers and I/O Registers                                    |
| `$4020` - `$5fff` | `$1fdf` | Expansion ROM - used by mappers to expand the capabilities of VRAM |
| `$6000` - `$7fff` | `$2000` | SRAM - Save Ram used to save data between game plays               |
| `$8000` - `$bfff` | `$4000` | PRG-ROM lower bank - executable code                               |
| `$c000` - `$ffff` | `$4000` | PRG-ROM upper bank - executable code (includes interrupt vectors)  |

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


| SIGNAL  | VECTOR          |
| ------- | --------------- |
| NMI     | `$FFFA`/`$FFFB` |
| RESET   | `$FFFC`/`$FFFD` |
| IRQ/BRK | `$FFFE`/`$FFFF` |

### Interrupt Handling

When a peripheral device pulls the interrupt request line low, the program counter
and status flags are pushed onto the stack, and control is transferred to the interrupt handling routine, located at the address stored in the appropriate
interrupt vector.

#### Resources

* High-level overview: [http://6502.org/tutorials/interrupts.html](http://6502.org/tutorials/interrupts.html)
* Detailed behavior: [https://wiki.nesdev.com/w/index.php/CPU_interrupts](https://wiki.nesdev.com/w/index.php/CPU_interrupts)
* Timing: [http://visual6502.org/wiki/index.php?title=6502_Timing_of_Interrupt_Handling](http://visual6502.org/wiki/index.php?title=6502_Timing_of_Interrupt_Handling)


## PPU

Picture processing unit. The NES used a 2C02 PPU, which is a [character generator](https://en.wikipedia.org/wiki/Character_generator) with sprites, designed by
Nintendo specifically for the NES.

A overview of NES graphics can be found at dustmop.io's "NES Graphics" parts [1](http://www.dustmop.io/blog/2015/04/28/nes-graphics-part-1/), [2](http://www.dustmop.io/blog/2015/06/08/nes-graphics-part-2/), and [3](http://www.dustmop.io/blog/2015/12/18/nes-graphics-part-3/).

### Registers

The PPU has 8 memory-mapped registers accessible by the CPU.

#### PPPUCTRL (`$2000`)

**Write-only**. Various flags controlling PPU operation.

| Bit position | Description                                                                                                     |
| ------------ | --------------------------------------------------------------------------------------------------------------- |
| `.... ..XX`  | Base nametable address.<br />(`0`: `$200`; `1`: `$2400`; `2`: `$2800`; `3`: `$2c00`)                            |
| `.... .X..`  | VRAM address increment per CPU read/write of PPUDATA.<br />(`0`: add 1, going across; `1`: add 32, going down.) |
| `.... X...`  | Sprite pattern table address for 8x8 spries.<br />(`0`: `$0000`; `1`: `$1000`; ignored in 8x16 mode)            |
| `...X ....`  | Background pattern table address (`0`: `$0000`; `1`: `$1000`)                                                   |
| `..X. ....`  | Sprite size (`0`: 8x8; `1`: 8x16)                                                                               |
| `.X.. ....`  | PPU master/slave select<br />(`0`: read background from EXT pins; `1`: output color on EXT pins)                |
| `X... ....`  | Generate an NMI at the start of the vertical blanking interval. (`0`: off; `1`: on)                             |

Equivalently, bits `0` and `1` are the most significant bit of the scrolling coordinates. If bit `0` is `1`, add 256 to the X scroll position.
If bit `1` is `1`, add 240 to the Y scroll position.

When turning on the NMI flag in bit 7, if the PPU is currently in vertical blank and the PPUSTATUS (`$2002`) vblank flag is set, an NMI will be generated immediately.

#### PPUMASK (`$2001`)

**Write-only**. Controls the rendering of sprites and backgrounds, as well as colour effects.

| Bit position | Description                                                     |
| ------------ | --------------------------------------------------------------- |
| `.... ...X`  | Greyscale (`0`: normal color; `1`: produce a greyscale display) |
| `.... ..X.`  | `1`: Show background in leftmost 8 pixels of screen; `0`: Hide  |
| `.... .X..`  | `1`: Show sprites in leftmost 8 pixels of screen; `0`: Hide     |
| `.... X...`  | `1`: Show background                                            |
| `...X ....`  | `1`: Show sprites                                               |
| `..X. ....`  | `1`: Emphasize red*                                             |
| `.X.. ....`  | `1`: Emphasize green*                                           |
| `X... ....`  | `1`: Emphasize blue*                                            |

* NTSC colors. PAL and Dendy swaps green and red.

Bit 0 controls a greyscale mode, which causes the palette to use only the colors from the grey column: `$00`, `$10`, `$20`, `$30`. This is implemented as a bitwise AND with `$30` on any value read from PPU `$3F00`-`$3FFF`, both on the display and through PPUDATA. Writes to the palette through PPUDATA are not affected. Also note that black colours like `$0F` will be replaced by a non-black grey `$00`.

Note that Sprite 0 hit does not trigger in any area where the background or sprites are hidden.

#### PPUSTATUS (`$2002`)

**Read-only**. Reflects the state of various functions inside the PPU. It is often used for determining timing.

*Bits 0-4* - The least significant bits previously written into a PPU register, due to the register not being updated for this address.

*Bit 5* - Sprite overflow. The intent was for this flag to be set whenever more than eight sprites appear on a scanline, but a hardware bug causes the actual behavior to be more complicated and generate false positives as well as false negatives; see [PPU sprite evaluation](https://wiki.nesdev.com/w/index.php/PPU_sprite_evaluation). This flag is set during sprite evaluation and cleared at dot 1 (the second dot) of the pre-render line.

*Bit 6* - Sprite 0 hit. Set when a nonzero pixel of sprite 0 overlaps a nonzero background pixel; cleared at dot 1 of the pre-render line. Used for raster timing.

*Bit 7* - Vertical blank has started (0: not in vblank; 1: in vblank). Set at dot 1 of line 241 (the line *after* the post-render line); cleared after reading `$2002` and at dot 1 of the pre-render line.

Note that Sprite 0 hit is not detected at x=255, nor is it detected at x=0 through 7 if the background or sprites are hidden in this area.

#### OAMADDR (`$2003`)

**Write-only**. Controls the address of OAM returned from the OAMDATA register.

Set to 0 during each of ticks 256-320 (the sprite loading interval) of the pre-render and visible scanlines.

#### OAMDATA (`$2004`)

**Read-write**. Read/write OAM data. Writes will increment OAMADDRR after the write; reads during vertical or forced blanking return the value from OAM at that address but do not increment.

#### PPUSCROLL (`$2005`)

**Write-only (double writes)**. Used to change the scroll position, that is, to tell the PPU which pixel of the nametable selected through PPUCTRL should be at the top left corner of the rendered screen.

Horizontal offsets range from 0 to 255. "Normal" vertical offsets range from 0 to 239, while values of 240 to 255 are treated as -16 through -1 in a way, but tile data is incorrectly fetched from the attribute table.

Registers PPUSCROLL and PPUADDR share a common write toggle, so that the first write has one behaviour, and the second write has another. After the second write, the toggle is reset to the first write behaviour. This toggle may be manually reset by reading PPUSTATUS.

#### PPUADDR (`$2006`)

**Write-only (double writes)**. Because the CPU and the PPU are on separate busses, neither has direct access to the other's memory. The CPU writes to VRAM through a pair of registers on the PPU. First it loads an address into PPUADDR, and then it writes repeatedly to PPUDATA to fill VRAM.

Value addresses are `$0000` - `$3fff`; higher addresses will be mirrored down.

#### PPUDATA (`$2007`)

**Read-write**. VRAM read/write data register. After access, the video memory address will increment by an amount determined by PPPUCTRL.

#### OAMDMA (`$4014`)

**Write-only**. This port is located on the CPU. Writing `$XX` will upload 256 bytes of data from CPU page `$XX00` - `XXff` to the internal PPU OAM. This page is typically located in internal RAM, commonly `$0200` - `$02ff`, but cartridge RAM or ROM can be used as well.

### Memory Map

The PPU addresses a 16kB space, `$0000` - `$3fff`, which is totally separate from the
CPU's address bus.

The NES has 2kB of RAM dedicated to the PPU, normally mapped to the nametable address
space from `$2000` - `$2fff`, but this can be remapped through custom cartridge wiring.

| Address Range     | Size    | Description                                                               |
| ----------------- | ------- | ------------------------------------------------------------------------- |
| `$0000` - `$0fff` | `$1000` | [Pattern table](https://wiki.nesdev.com/w/index.php/PPU_pattern_tables) 0 |
| `$1000` - `$1fff` | `$1000` | [Pattern table](https://wiki.nesdev.com/w/index.php/PPU_pattern_tables) 1 |
| `$2000` - `$23ff` | `$0400` | [Nametable](https://wiki.nesdev.com/w/index.php/PPU_nametables) 0         |
| `$2400` - `$27ff` | `$0400` | [Nametable](https://wiki.nesdev.com/w/index.php/PPU_nametables) 1         |
| `$2800` - `$2bff` | `$0400` | [Nametable](https://wiki.nesdev.com/w/index.php/PPU_nametables) 2         |
| `$2c00` - `$2fff` | `$0400` | [Nametable](https://wiki.nesdev.com/w/index.php/PPU_nametables) 3         |
| `$3000` - `$3eff` | `$0f00` | Mirror of `$2000` - `$2eff`                                               |
| `$3f00` - `$3f1f` | `$0020` | [Palette RAM](https://wiki.nesdev.com/w/index.php/PPU_palettes) indexes   |
| `$3f20` - `$3fff` | `$00e0` | Mirror of `$3f00` - `$3f1f`                                               |

In addition, the PPU contains 256 bytes of memory known as [Object Attribute Memory](https://wiki.nesdev.com/w/index.php/PPU_OAM) which determines how sprites are rendered. The CPU can manipulate
this memory the memory mapped registers at [OAMADDR](https://wiki.nesdev.com/w/index.php/PPU_registers#OAMADDR) (`$2003`), [OAMDATA](https://wiki.nesdev.com/w/index.php/PPU_registers#OAMDATA) (`$2004`), and [OAMDMA](https://wiki.nesdev.com/w/index.php/PPU_registers#OAMDMA) (`$4014`).

| Address Range          | Size  | Description         |
| ---------------------- | ----- | ------------------- |
| `$00` - `$0c` (0 of 4) | `$40` | Sprite Y coordinate |
| `$01` - `$0d` (1 of 4) | `$40` | Sprite tile #       |
| `$02` - `$0e` (2 of 4) | `$40` | Sprite attribute    |
| `$03` - `$0f` (3 of 4) | `$40` | Sprite X coordinate |

#### Hardware mapping

The mappings above are the fixed addresses that the PPU fetches data during rendering. However, the actual
device from which the data is fetched may be configured by the cartridge.

*   `$0000` - `$1fff` is normally mapped by the cartridge to [CHR-ROM]() or [CHR-RAM](), often with a bank
    switching mechanism.
*   `$2000` - `$2fff` is normally mapped to the 2kB NES internal VRAM, providing 2 nametables with a mirroring
    configuration controlled by the cartridge, but it can be partly or fulled remapped to RAM on the cartridge,
    allowing up to 4 simultaneous nametables.
*   `$3000` - `$3eff` is usually a mirror of the 2kB region from `$2000` - `$2eff`. The PPU does not render from
    this address range, so this space has negligble utility.
*   `$3f00` - `$3fff` is not configurable, always mapped to the internal palette control.

### Rendering

The PPU renders 262 scanlines per frame (although only 240 scanlines are visible on the screen). Each scanline lasts for 341 PPU clock cycles (113.667 CPU clock cycles; 1 CPU cycle = 3 PPU cycles), with each clock cycle producing one pixel.

### Scrolling


## APU

The NES has an audio processing unit for generating sound in games. It is implemented in the RP2A03 (NTSC) and RP2A07 (PAL) chips. A good overview can be found at [nesdev.com](http://wiki.nesdev.com/w/index.php/APU).


### Channels

The APU has five channels: two pulse waves, triangle wave, noise, and DMC (sampling).

The channel registers begin at `$4000`, and each channel has four registers devoted to it. All but the triangle wave have 4-bit volume control (the triangle just has a mute/unmute flag).

*   `$4000` - `$4003`: Pulse wave 1
*   `$4004` - `$4007`: Pulse wave 2
*   `$4008` - `$400b`: Triangle wave
*   `$400c` - `$400f`: Noise
