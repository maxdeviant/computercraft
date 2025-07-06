use std::fmt::Display;
use std::sync::Arc;

#[derive(Debug, Eq, Clone)]
pub enum BlockId {
    Static(&'static str),
    Owned(Arc<str>),
}

impl BlockId {
    pub const fn new_static(id: &'static str) -> Self {
        Self::Static(id)
    }

    pub fn new(id: &str) -> Self {
        Self::Owned(Arc::from(id))
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Static(id) => id,
            Self::Owned(id) => id.as_ref(),
        }
    }
}

impl PartialEq for BlockId {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Display for BlockId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Block {
    pub id: BlockId,
}

impl Block {
    pub fn is_solid(&self) -> bool {
        self.id != BlockId::AIR
    }

    pub fn is_diggable(&self) -> bool {
        self.id != BlockId::AIR && self.id != BlockId::BEDROCK
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_block_id_equality() {
        assert_eq!(
            BlockId::new_static("minecraft:air"),
            BlockId::new("minecraft:air")
        );
        assert_eq!(
            BlockId::new_static("minecraft:stone"),
            BlockId::new("minecraft:stone")
        );

        assert_ne!(
            BlockId::new_static("minecraft:air"),
            BlockId::new_static("minecraft:stone")
        );
    }
}
