use plonky2::{iop::target::BoolTarget, plonk::circuit_builder::CircuitBuilder, hash::hash_types::RichField};
use plonky2_field::extension::Extendable;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2_u32::gadgets::arithmetic_u32::U32Target;

pub struct BlockIDTarget {
    pub hash: [U32Target; 8],
    pub partset_total: U32Target,
    pub partset_hash: [U32Target; 8],
}