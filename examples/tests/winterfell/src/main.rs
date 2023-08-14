use std::fs::File;
use std::{io, process};
use std::io::Read;
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
use log::info;
use std::env;

mod example;

pub struct ExampleProver<H: ElementHasher> {
    options: WinterProofOptions,
    _hasher: PhantomData<H>,
}

impl<H: ElementHasher> ExampleProver<H> {
    pub fn new(options: WinterProofOptions) -> Self {
println!("RUNNNING WINTERFELL");
        Self {
            options,
            _hasher: PhantomData,
        }
    }
    /// Builds an execution trace of the air
    pub fn build_trace(&self, table_f: &str, inputs: &[Felt; 16]) -> TraceTable<Felt> {
        fn parse_fast(value: &str) -> u64 {
            let mut result = 0u64;
            for b in value.bytes() {
                if b >= 48  && b < 58 { 
                    result = 10u64 * result + ((b as u64) - 48u64);
                }
                else { panic!("Invalid character in trace table"); }
            }
            result
        }
/*
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
        let tab = vec![s,a,b,c];
        println!("TRACE TABLE {:?}", tab);
*/
        let mut table_file = File::open (table_f).unwrap();
        let mut table_data = String::new(); 
        table_file.read_to_string(&mut table_data).unwrap();
        println!("RAW TABLE {:?}",&mut table_data);

        let lines = table_data.split("\n");
        let mut vwords = Vec::<Vec::<u64>>::new();
        for (lno, line) in lines.into_iter().enumerate() {
           //println!("{:?} -- {:?}", lno, line);
           if line != "" {
             let mut vs = Vec::<String>::new();
             let words = line.split(" ");
             let mut vw = Vec::<u64>::new(); 
             for word in words.into_iter() {
               if word != "" {
                let k = parse_fast(&word);
                 vw.push(parse_fast(&word));
                 //println!("Add word: `{:?}`", k);
               }
             } 
             vwords.push(vw);
           }
        }
        let nrows = vwords.len();
        let ncols = vwords[0].len();

        println!("Trace length {:?}",nrows);
        assert!(
            nrows.is_power_of_two(),
            "sequence length must be a power of 2"
        );

        println!("Trace width {:?}",ncols);
        println!("DATA {:?}",vwords);
        let mut tab = Vec::<Vec::<Felt>>::new();
        for colix in 0..ncols {
          let mut col = Vec::<Felt>::new();
          for rowix in 0..nrows {
            col.push(Felt::from(vwords[rowix][colix]));   
          }
          tab.push(col);
        }
   
        TraceTable::init(tab)

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
        let last_step = trace.length() - 1; // why is this 2?
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
    let args: Vec<String> = env::args().collect(); 
    let trace_f = &args[1];

    let inputs = [Felt::ONE; 16];
    let options = WinterProofOptions::new(27, 8, 16, FieldExtension::None, 8, 255);
    let prover = ExampleProver::<Blake3_192<Felt>>::new(options);
    let trace = prover.build_trace(trace_f, &inputs);
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

