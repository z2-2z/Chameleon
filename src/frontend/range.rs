
pub type SourceRange = std::ops::Range<usize>;

pub trait NewRange<Idx> {
    fn new(start: Idx, end: Idx) -> Self;
}

impl<Idx> NewRange<Idx> for std::ops::Range<Idx> {
    fn new(start: Idx, end: Idx) -> Self {
        Self {
            start,
            end
        }
    }
}


