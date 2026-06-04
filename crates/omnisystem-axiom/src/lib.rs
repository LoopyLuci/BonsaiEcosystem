pub mod proof_checker;
pub mod hints;
pub mod attributes;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AxiomProof {
    pub name: String,
    pub statement: String,
    pub proof_script: String,
    pub verified: bool,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiedProperty {
    pub property_type: PropertyType,
    pub description: String,
    pub proof_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyType {
    NoPanic, BoundedTime(u64), NoAllocation, Idempotent,
    Commutative, Associative, RoundTrip, Custom(String),
}
