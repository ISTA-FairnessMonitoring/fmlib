use super::{
    memoryless::{Sample, Decision, Payback},
    memoryless::Vertex
};

pub type Lmv = LendingMappedVertex;

#[derive(Clone, PartialEq, Eq, Debug, Default, Hash)]
pub enum LendingMappedVertex {
    Sample(Sample),
    Decision(Sample, Decision),
    Payback(Sample, Payback),
    #[default]
    Null,
}

pub struct LendingVertexMapper;

impl LendingVertexMapper {
    pub fn map(&self, s: &Vertex) -> Option<Lmv> {
        match s {
            Vertex::Sample(s) => { Some( Lmv::Sample(*s) ) },
            Vertex::Decision(s, d) => {
                Some( Lmv::Decision(*s, *d) )
            },
            Vertex::Payback(s, _, p) => {
                Some( Lmv::Payback(*s, *p) )
            },
            _ => None,
        }
    }
}
