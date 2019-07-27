use super::engine::*;

pub struct Ghost<'g> {
    war: &'g War,
}

impl<'g> Ghost<'g> {
    pub fn new(war: &'g War) -> Ghost<'g> {
        Ghost { war: war }
    }

    pub fn evaluate(&self) -> i32 {
        0
    }
}
