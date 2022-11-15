use super::{lv::{Sample, Decision, Payback}, memoryless::Vertex};

pub type Lmv = LendingMappedVertex;

#[derive(Clone, PartialEq, Eq, Debug, Default, Hash)]
pub enum LendingMappedVertex {
    Sample(Sample),
    Decision(Sample, Decision),
    Payback(Sample, Payback),
    #[default]
    Null,
}

pub type Lmvm = LendingMCVertexMapper;
pub struct LendingMCVertexMapper;

impl LendingMCVertexMapper {
    pub fn map(&self, s: &Vertex) -> Option<Lmv> {
        match s {
            Vertex::Sample(_, s) => Some(Lmv::Sample(*s)),
            Vertex::Decision(_, s, d) => Some(Lmv::Decision(*s, *d)),
            Vertex::Payback(_, s, _, p) => Some(Lmv::Payback(*s, *p)),
            _ => None,
        }
    }
}
