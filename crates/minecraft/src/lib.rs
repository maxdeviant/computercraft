pub mod world;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Block {
    Air,
    Stone,
    Dirt,
    Wood,
    Cobblestone,
    Bedrock,
}

impl Block {
    pub fn is_solid(&self) -> bool {
        !matches!(self, Block::Air)
    }

    pub fn is_diggable(&self) -> bool {
        !matches!(self, Block::Air | Block::Bedrock)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemStack {
    pub name: String,
    pub count: u32,
}

impl ItemStack {
    pub fn new(name: String, count: u32) -> Self {
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
