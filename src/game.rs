use bytecode::instructions::*;
use common::*;

#[inline]
pub fn update_and_render(framebuffer: &mut Framebuffer, state: &mut GameState, input: Input) {
    for i in 0..state.turtles.len() {
        state.interpret(framebuffer, i);
    }
}
