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
                ($colour & 0xFF_FF_00_FF) | $green << 8
            };
            (blue, $colour:expr, $blue:expr) => {
                ($colour & 0xFF_00_FF_FF) | $blue << 16
            };
            (grey, $colour:expr, $grey:expr) => {{
                let bottom = $grey & 0xFF;

                ($colour & 0xFF_00_00_00) | bottom | bottom << 8 | bottom << 16
            }};
        }

        macro_rules! op_as_u8 {
            ($name:ident, $x:expr, $code:expr) => {{
                let $name = ($x) as u8;

                ($code) as u32
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

        match turtle_index {
            0 => {
                *c = set!(
                    red,
                    *c,
                    op_as_u8!(
                        x,
                        red!(*c),
                        if x == 0 {
                            1
                        } else {
                            x.saturating_mul(2)
                        }
                    )
                );
            }
            1 => {
                *c = set!(
                    green,
                    *c,
                    op_as_u8!(
                        x,
                        green!(*c),
                        if x == 0 {
                            1
                        } else {
                            x.saturating_mul(2)
                        }
                    )
                );
            }
            _ => {
                *c = set!(
                    blue,
                    *c,
                    op_as_u8!(
                        x,
                        blue!(*c),
                        if x == 0 {
                            1
                        } else {
                            x.saturating_mul(2)
                        }
                    )
                );
            }
        }
    }
}
