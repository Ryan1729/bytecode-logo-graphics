use common::{Framebuffer, GameState};

pub mod instructions {
    pub const UP: u8 = 0;
    pub const DOWN: u8 = 1;
    pub const LEFT: u8 = 2;
    pub const RIGHT: u8 = 3;

    pub const RED_INC: u8 = 4;
    pub const RED_DEC: u8 = 5;
    pub const GREEN_INC: u8 = 6;
    pub const GREEN_DEC: u8 = 7;
    pub const BLUE_INC: u8 = 8;
    pub const BLUE_DEC: u8 = 9;

    pub const RED_SHL: u8 = 10;
    pub const RED_SHR: u8 = 11;
    pub const GREEN_SHL: u8 = 12;
    pub const GREEN_SHR: u8 = 13;
    pub const BLUE_SHL: u8 = 14;
    pub const BLUE_SHR: u8 = 15;
}
pub use self::instructions::*;

impl GameState {
    pub fn interpret(&mut self, framebuffer: &mut Framebuffer, turtle_index: usize) {
        let bytecode = self.instructions[turtle_index];
        let len = bytecode.len();
        let mut i = 0;
        while i < len {
            let instruction = bytecode[i];

            self.interpret_instruction(framebuffer, turtle_index, instruction & 0b1111);

            self.interpret_instruction(framebuffer, turtle_index, instruction >> 4);

            i += 1;
        }
    }

    #[inline]
    pub fn interpret_instruction(
        &mut self,
        framebuffer: &mut Framebuffer,
        turtle_index: usize,
        instruction: u8,
    ) {
        let turtle = &mut self.turtles[turtle_index];
        macro_rules! buffer_index {
            () => {
                turtle.0 as usize + turtle.1 as usize * 256
            };
        }

        macro_rules! set {
            (red, $colour:expr, $red:expr) => {
                ($colour & 0xFF_FF_FF_00) | $red
            };
            (green, $colour:expr, $green:expr) => {
                ($colour & 0xFF_FF_00_FF) | $green
            };
            (blue, $colour:expr, $blue:expr) => {
                ($colour & 0xFF_00_FF_FF) | $blue
            };
        }

        match instruction {
            UP => {
                turtle.1 = turtle.1.saturating_add(1);
            }
            DOWN => {
                turtle.1 = turtle.1.saturating_sub(1);
            }
            LEFT => {
                turtle.0 = turtle.0.saturating_add(1);
            }
            RIGHT => {
                turtle.0 = turtle.0.saturating_sub(1);
            }

            RED_INC => {
                let buffer_index = buffer_index!();
                let c = &mut framebuffer.buffer[buffer_index];
                *c = set!(red, *c, red!(*c) + 1);
            }
            RED_DEC => {
                let buffer_index = buffer_index!();
                let c = &mut framebuffer.buffer[buffer_index];
                *c = set!(red, *c, red!(*c) - 1);
            }
            GREEN_INC => {
                let buffer_index = buffer_index!();
                let c = &mut framebuffer.buffer[buffer_index];
                *c = set!(green, *c, green!(*c) + 1);
            }
            GREEN_DEC => {
                let buffer_index = buffer_index!();
                let c = &mut framebuffer.buffer[buffer_index];
                *c = set!(green, *c, green!(*c) - 1);
            }
            BLUE_INC => {
                let buffer_index = buffer_index!();
                let c = &mut framebuffer.buffer[buffer_index];
                *c = set!(blue, *c, blue!(*c) + 1);
            }
            BLUE_DEC => {
                let buffer_index = buffer_index!();
                let c = &mut framebuffer.buffer[buffer_index];
                *c = set!(blue, *c, blue!(*c) - 1);
            }

            RED_SHL => {
                let buffer_index = buffer_index!();
                let c = &mut framebuffer.buffer[buffer_index];
                *c = set!(red, *c, red!(*c) << 1);
            }
            RED_SHR => {
                let buffer_index = buffer_index!();
                let c = &mut framebuffer.buffer[buffer_index];
                *c = set!(red, *c, red!(*c) >> 1);
            }
            GREEN_SHL => {
                let buffer_index = buffer_index!();
                let c = &mut framebuffer.buffer[buffer_index];
                *c = set!(green, *c, green!(*c) << 1);
            }
            GREEN_SHR => {
                let buffer_index = buffer_index!();
                let c = &mut framebuffer.buffer[buffer_index];
                *c = set!(green, *c, green!(*c) >> 1);
            }
            BLUE_SHL => {
                let buffer_index = buffer_index!();
                let c = &mut framebuffer.buffer[buffer_index];
                *c = set!(blue, *c, blue!(*c) << 1);
            }
            BLUE_SHR => {
                let buffer_index = buffer_index!();
                let c = &mut framebuffer.buffer[buffer_index];
                *c = set!(blue, *c, blue!(*c) >> 1);
            }
            _ => {}
        }
    }
}
