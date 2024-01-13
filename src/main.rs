use std::{iter, array};
use std::iter::{zip, Enumerate};
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::BoolTarget;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2_field::extension::Extendable;
use plonky2::plonk::circuit_data::{
    CircuitConfig, CommonCircuitData, VerifierCircuitTarget, VerifierOnlyCircuitData,
};
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig, Hasher, PoseidonGoldilocksConfig};
use plonky2_sha256;

mod dualmux;
pub struct DualHashTarget {
    input0: [BoolTarget; 256],
    input1: [BoolTarget; 256],
    selector: BoolTarget,
    output: [BoolTarget; 256],
}

pub fn make_dualhash<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>
) -> DualHashTarget {
    let mut input0 = Vec::new();
    let mut input1 = Vec::new();
    let mut output0 = Vec::new();
    let selector = builder.add_virtual_bool_target_safe();
    for _ in 0..256 {
        let i0 = builder.add_virtual_bool_target_safe();
        let i1 = builder.add_virtual_bool_target_safe();
        let o0 = builder.add_virtual_bool_target_safe();
        input0.push(i0);
        input1.push(i1);
        output0.push(o0);
    }
    let mux = dualmux::make_multiplexer(builder);

    for i in 0..256 {
        builder.connect(input0[i].target, mux.input0[i].target);
        builder.connect(input1[i].target, mux.input1[i].target);
    }
    builder.connect(selector.target, mux.selector.target);

    let hasher = plonky2_sha256::circuit::make_circuits(builder, 512);
    for i in 0..256 {
        builder.connect(mux.output0[i].target, hasher.message[i].target);
    }
    for i in 0..256 {
        builder.connect(mux.output1[i].target, hasher.message[255+i].target);
    }

    DualHashTarget { 
        input0: input0.try_into().unwrap(), 
        input1: input1.try_into().unwrap(), 
        selector: selector, 
        output: hasher.digest.try_into().unwrap()
    }
}

fn main()  {
}