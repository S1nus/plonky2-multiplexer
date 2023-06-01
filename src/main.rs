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

mod dualmux;


fn main()  {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;
    let mut builder = CircuitBuilder::<F, D>::new(CircuitConfig::standard_recursion_config());
    let targets = dualmux::make_multiplexer(&mut builder);
    let mut pw = PartialWitness::new();
    let input0: [u8; 32] = [0; 32];
    let input1: [u8; 32] = [1; 32];
    dualmux::fill_circuits::<F, D>(&mut pw, input0, input1, false, targets);
    println!(
        "Constructing proof with {} gates",
        builder.num_gates()
    );
    let data = builder.build::<C>();
    println!("Proving...");
    let proof = data.prove(pw).unwrap();
    println!("Done proving!");
    println!("Verifying proof...");
    data.verify(proof.clone()).expect("verify error");
    println!("Done verifying!");
}
