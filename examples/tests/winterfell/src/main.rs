
use std::fs::File;
use std::marker::PhantomData;

use example::{ExampleAir, PublicInputs};
use log::LevelFilter;
use std::io::Write;
use winter_air::{FieldExtension, ProofOptions as WinterProofOptions};
use winter_math::{fields::f64::BaseElement as Felt, FieldElement};
use winter_prover::crypto::hashers::Blake3_192;
use winter_prover::crypto::{DefaultRandomCoin, ElementHasher};
use winter_prover::{Prover, Trace, TraceTable};
use winter_verifier::verify;

mod example;

pub struct ExampleProver<H: ElementHasher> {
    options: WinterProofOptions,
    _hasher: PhantomData<H>,
}

impl<H: ElementHasher> ExampleProver<H> {
    pub fn new(options: WinterProofOptions) -> Self {
        Self {
            options,
            _hasher: PhantomData,
        }
    }

    /// Builds an execution trace of the air
    pub fn build_trace(&self, sequence_length: usize, inputs: &[Felt; 16]) -> TraceTable<Felt> {
        assert!(
            sequence_length.is_power_of_two(),
            "sequence length must be a power of 2"
        );

        let mut s = vec![Felt::ONE];
        let mut a = vec![inputs[0]];
        let mut b = vec![inputs[1]];
        let mut c = vec![inputs[2]];

        for i in 1..sequence_length {
            let new_a = Felt::from((i + 1) as u64);
            let new_b = Felt::from((i + 2) as u64);
            s.push(Felt::ONE); // selector stays the same
            a.push(new_a);
            b.push(new_b);
            c.push(new_a * new_b); // c is a * b
        }

        TraceTable::init(vec![s, a, b, c])
    }
}

impl<H: ElementHasher> Prover for ExampleProver<H>
where
    H: ElementHasher<BaseField = Felt>,
{
    type BaseField = Felt;
    type Air = ExampleAir;
    type Trace = TraceTable<Felt>;
    type HashFn = H;
    type RandomCoin = DefaultRandomCoin<Self::HashFn>;

    fn get_pub_inputs(&self, trace: &Self::Trace) -> PublicInputs {
        // based on air, c = a * b and therefore all 1s
        let inputs = [Felt::ONE; 16];
        let mut outputs = [Felt::ONE; 16];
        let last_step = trace.length() - 2; // why is this 2?
        outputs[0] = trace.get(1, last_step); // a
        outputs[1] = trace.get(2, last_step); // b
        outputs[2] = trace.get(3, last_step); // c
        PublicInputs::new(inputs, outputs)
    }

    fn options(&self) -> &WinterProofOptions {
        &self.options
    }
}

fn main() {
    let inputs = [Felt::ONE; 16];
    let options = WinterProofOptions::new(27, 8, 16, FieldExtension::None, 8, 255);
    let prover = ExampleProver::<Blake3_192<Felt>>::new(options);
    let trace = prover.build_trace(8, &inputs);
    let pub_inputs = prover.get_pub_inputs(&trace);
    let proof = prover.prove(trace).unwrap();
    let example_log = File::create("example.wlog").unwrap();

    // only print verifier
    env_logger::Builder::new()
        .format_timestamp(None)
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .filter(None, LevelFilter::Info)
        .target(env_logger::Target::Pipe(Box::new(example_log)))
        .init();
    verify::<ExampleAir, Blake3_192<Felt>, DefaultRandomCoin<Blake3_192<Felt>>>(proof, pub_inputs)
        .unwrap();
}
