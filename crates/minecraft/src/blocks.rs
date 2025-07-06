use std::sync::LazyLock;

use crate::{Block, BlockId};

impl BlockId {
    pub const AIR: Self = Self::Static("minecraft:air");
    pub const BEDROCK: Self = Self::Static("minecraft:bedrock");
    pub const GRASS_BLOCK: Self = Self::Static("minecraft:grass_block");
    pub const DIRT: Self = Self::Static("minecraft:dirt");
    pub const FARMLAND: Self = Self::Static("minecraft:farmland");
    pub const STONE: Self = Self::Static("minecraft:stone");
}

pub static AIR: LazyLock<Block> = LazyLock::new(|| Block { id: BlockId::AIR });
pub static BEDROCK: LazyLock<Block> = LazyLock::new(|| Block {
    id: BlockId::BEDROCK,
});
pub static GRASS_BLOCK: LazyLock<Block> = LazyLock::new(|| Block {
    id: BlockId::GRASS_BLOCK,
});
pub static DIRT: LazyLock<Block> = LazyLock::new(|| Block { id: BlockId::DIRT });
pub static FARMLAND: LazyLock<Block> = LazyLock::new(|| Block {
    id: BlockId::FARMLAND,
});
pub static STONE: LazyLock<Block> = LazyLock::new(|| Block { id: BlockId::STONE });
