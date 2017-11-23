Notes on emulator implementation, options, trade-offs, etc.

<!-- TOC -->

* [Memory Mapping Options](#memory-mapping-options)
  * [nintengo](#nintengo)
  * [nes-rs](#nes-rs)
  * [sprocketnes](#sprocketnes)
  * [oxidenes](#oxidenes)

<!-- /TOC -->

## Memory Mapping Options

### nintengo

    * MappedMemory object has mappings from addr => "Memory" implementation for
      every address.
    * MemoryMapping object defines which fetch/store addresses should be mapped.
      The idea is that each of PPU, APU, and all the mappers would implement
      MemoryMapping
    * The issue is that this requires MappedMemory to hold mutable references
      to all MemoryMapping objects, so that it can call fetch/store on them
      when required. This conflicts with Rust's idea of single mutable
      ownsership.
    * Potential solution: MappedMemory holds a Rc<RefCell<Memory>>. This gets
      around the compiler issues of single mutable ownership, with a runtime
      overhead.

### nes-rs

    * All PPU memory and registers are hardcoded into the Memory class.
    * Mapping are just a match over hardcoded PPU addresses.
    * Super simple, easy to implement
    * Probably not going to extend well to APU and mapper mappings.

### sprocketnes

    * Memory object owns the PPU object, as a field on the Memory struct.
    * All other accesses to the PPU are through "cpu.mem.ppu"
    * Downside is this is much more tightly-coupled.
    * Mapper object is shared between Memory object and PPU object, via
      Rc<RefCell<Box<Mapper+Send>>>.
    * Why not just do that same sharing model for the Memory owning the PPU?

### oxidenes

    * All memory ops are defined on the CPU object.
    * PPU and APU address mappings are hardcoded in the write/read functions.
    * PPU/APU/Mapper are owned by a "Bus" object.
    * "Bus" object is owned by the CPU.
    * So, the CPU "write" method looks like:
      "match addr { ... PPUDATA_ADDR => self.bus.ppu.write_ppudata(value) }".
    * Similar match arms for APU/Mapper.
