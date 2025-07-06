use std::sync::LazyLock;

use crate::{Block, BlockId};

impl BlockId {
    pub const AIR: Self = Self::Static("minecraft:air");
    pub const BEDROCK: Self = Self::Static("minecraft:bedrock");
    pub const DIRT: Self = Self::Static("minecraft:dirt");
    pub const STONE: Self = Self::Static("minecraft:stone");
}

pub static AIR: LazyLock<Block> = LazyLock::new(|| Block { id: BlockId::AIR });
pub static BEDROCK: LazyLock<Block> = LazyLock::new(|| Block {
    id: BlockId::BEDROCK,
});
pub static DIRT: LazyLock<Block> = LazyLock::new(|| Block { id: BlockId::DIRT });
pub static STONE: LazyLock<Block> = LazyLock::new(|| Block { id: BlockId::STONE });
