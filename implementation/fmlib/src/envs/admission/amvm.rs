// Interface between the vertices of the admission MC and the monitor

use super::memoryless::Vertex;

pub type Amv = AdmissionMappedVertex;

#[derive(Clone, PartialEq, Debug, Default, Eq, Hash)]
pub enum AdmissionMappedVertex {
    Sample(bool),
    Cost(i32),
    #[default]
    Null,
}

pub type Amvm = AdmissionMCVertexMapper;
pub struct AdmissionMCVertexMapper;

impl AdmissionMCVertexMapper {
    pub fn map(&self, s: &Vertex) -> Option<Amv> {
        match s {
            Vertex::Sample(a) => { Some(Amv::Sample(a.label)) }
            Vertex::Cost(_, c) => { Some(Amv::Cost(*c)) }
            _ => None,
        }
    }
}
