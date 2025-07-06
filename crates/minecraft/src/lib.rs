mod block;
pub mod blocks;
mod item;
pub mod world;

pub use block::*;
pub use item::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemStack {
    pub name: ItemId,
    pub count: u32,
}

impl ItemStack {
    pub fn new(name: ItemId, count: u32) -> Self {
        Self { name, count }
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn max_stack_size(&self) -> u32 {
        64
    }

    pub fn space_left(&self) -> u32 {
        self.max_stack_size().saturating_sub(self.count)
    }
}
