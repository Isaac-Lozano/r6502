//! This library is an emulator for the 6502 CPU. It includes support for a
//! generic memory mapper so that many different systems can be emulated with
//! this base.
//!
//! # Examples
//! Basic usage is:
//!
//! ```no_run
//! extern crate r6502;
//! use r6502::cpu6502::Cpu6502;
//! use r6502::memory::SimpleMemory;
//!
//! fn main()
//! {
//!     let mut mem = SimpleMemory::new();
//!     /* Initialize memory */
//!     let mut cpu = Cpu6502::new(mem);
//!     loop
//!     {
//!         cpu.run(1).unwrap();
//!     }
//! }
//! ```

pub mod cpu6502;
pub mod memory;
pub mod opcode;

#[macro_use]
extern crate bitflags;

#[cfg(test)]
mod tests {
    use cpu6502::Cpu6502;
    use memory::{SimpleMemory, Memory};

    use std::io::prelude::*;
    use std::fs::File;
    use std::error::Error;

    #[test]
    fn cpu_simple_test() {
        let mut mem = SimpleMemory::new();

        /* Set reset vector */
        mem.write(0xFFFD, 0xFF as u8);
        mem.write(0xFFFF, 0xFC as u8);

        /* adc #$42 */
        mem.write(0xFF00, 0x69 as u8);
        mem.write(0xFF01, 0x42 as u8);

        /* adc $FF00 ; ($69)*/
        mem.write(0xFF02, 0x6D as u8);
        mem.write(0xFF03, 0x00 as u8);
        mem.write(0xFF04, 0xFF as u8);

        let mut cpu = Cpu6502::new(mem);
        cpu.reset();
        println!("{:?}\n", cpu);
        cpu.run(1).unwrap();
        println!("{:?}\n", cpu);
        assert!(cpu.a == 0x42);
        cpu.run(1).unwrap();
        println!("{:?}\n", cpu);
        assert!(cpu.a == 0xAB);
    }

    #[test]
    fn cpu_full_test() {
        let mut mem = SimpleMemory::new();
        let mut file = File::open("test_data/6502_functional_test.bin").unwrap();
        file.read(&mut mem.mem).unwrap();
        let mut cpu = Cpu6502::new(mem);

        let mut cycle_count = 0;
        let mut last_pc = 0x0000;
        cpu.pc = 0x1000;
        while last_pc != cpu.pc
        {
            last_pc = cpu.pc;
            match cpu.run(1)
            {
                Ok(cycles) =>
                    cycle_count += cycles,
                Err(err) =>
                {
                    println!("Error {}: {}", err, err.description());
                    assert!(false);
                }
            }
        }
        println!("Program exited normally after {} cycles.", cycle_count);
    }
}
