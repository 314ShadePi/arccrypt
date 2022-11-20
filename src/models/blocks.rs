use super::block::Block;

pub struct Blocks(pub Vec<Block>);

impl std::fmt::Display for Blocks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.iter().fold(Ok(()), |result, block| {
            result.and_then(|_| writeln!(f, "{}", block))
        })
    }
}

impl std::ops::Deref for Blocks {
    type Target = Vec<Block>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Blocks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
