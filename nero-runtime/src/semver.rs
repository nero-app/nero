#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct SemanticVersion {
    major: usize,
    minor: usize,
    patch: usize,
}

impl SemanticVersion {
    pub const fn new(major: usize, minor: usize, patch: usize) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}
