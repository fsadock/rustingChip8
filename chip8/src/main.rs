use std::fmt::DebugList;

mod chip {

    const MEM_SIZE: usize = 4096;
    const NUM_REGS: usize = 16;
    const FONT_SIZE: usize = 80;

    pub const FONT: [u8; FONT_SIZE] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80, // F
    ];

    #[derive(Debug)]
    pub struct ChipEight {
        pub memory: [u8; MEM_SIZE],
        pub display: [[u8; 32]; 64],
        pub pc: u16,
        pub i_reg: u16,
        pub stack: [u16; 10],
        pub delay_timer: u8,
        pub sound_timer: u8,
        pub registers: [u8; NUM_REGS],
        // keypad: [u8; 16],
    }

    impl ChipEight {
        pub fn new() -> ChipEight {
            ChipEight {
                memory: [0; 4096],
                display: [[0; 32]; 64],
                pc: 0,
                i_reg: 0,
                stack: [11; 10],
                delay_timer: 15,
                sound_timer: 16,
                registers: [2; 16],
            }
        }
    }
}
fn main() {
    let mut test = chip::ChipEight::new();

    for n in 0..=79 {
        test.memory[n] = chip::FONT[n];
    }

    println!("{:?}", &test.memory[0..79]);
    // println!("{:?}", FONT);
}
