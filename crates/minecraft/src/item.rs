use std::fmt::Display;
use std::sync::Arc;

#[derive(Debug, Eq, Clone)]
pub enum ItemId {
    Static(&'static str),
    Owned(Arc<str>),
}

impl ItemId {
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

impl PartialEq for ItemId {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Display for ItemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
