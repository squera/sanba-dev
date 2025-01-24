pub mod full_tables;
pub mod insertions;
pub mod others;

/// Questo trait permette a una struttura dati senza id di venir convertita alla corrispondente struttura con id
pub trait WithId {
    type IdentifiedType;
    type IdType;

    fn to_identified(&self, id: Self::IdType) -> Self::IdentifiedType;
}
