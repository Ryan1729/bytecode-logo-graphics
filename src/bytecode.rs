use common::{Framebuffer, GameState};

pub mod instructions {
    pub const UP: u8 = 0;
    pub const DOWN: u8 = 1;
    pub const LEFT: u8 = 2;
    pub const RIGHT: u8 = 3;
}
pub use self::instructions::*;

impl GameState {
    pub fn interpret(&mut self, framebuffer: &mut Framebuffer, turtle_index: usize) {
        let bytecode = self.instructions[turtle_index];
        let len = bytecode.len();
        let mut i = 0;
        while i < len {
            let instruction = bytecode[i];

            self.interpret_instruction(framebuffer, turtle_index, instruction & 0b11);

            self.interpret_instruction(framebuffer, turtle_index, (instruction & 0b1100) >> 2);

            self.interpret_instruction(framebuffer, turtle_index, (instruction & 0b110000) >> 4);

            self.interpret_instruction(framebuffer, turtle_index, (instruction & 0b11000000) >> 6);

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
            (grey, $colour:expr, $grey:expr) => {{
                let bottom = $grey & 0xFF;

                ($colour & 0xFF_00_00_00) | bottom | bottom << 8 | bottom << 16
            }};
        }

        match instruction {
            UP => {
                turtle.1 = turtle.1.wrapping_add(1);
            }
            DOWN => {
                turtle.1 = turtle.1.wrapping_sub(1);
            }
            LEFT => {
                turtle.0 = turtle.0.wrapping_add(1);
            }
            RIGHT => {
                turtle.0 = turtle.0.wrapping_sub(1);
            }
            _ => {}
        }

        let buffer_index = buffer_index!();
        let c = &mut framebuffer.buffer[buffer_index];
        let current = *c;
        let new_c = if current & 1 == 0 {
            if current >= 254 {
                255
            } else {
                (current).saturating_add(2)
            }
        } else {
            if current <= 1 {
                0
            } else {
                (current).saturating_sub(2)
            }
        };

        *c = set!(grey, *c, new_c);
    }
}
