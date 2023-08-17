
use std::fs::File;
use std::{io, process};
use std::io::Read;
use std::marker::PhantomData;
use example::{FibAir, PublicInputs};
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

fn parse_fast(value: &str) -> Felt {
  let mut result = 0u64;
  for b in value.bytes() {
    if b >= 48  && b < 58 { 
      result = 10u64 * result + ((b as u64) - 48u64);
    }
    else { panic!("Invalid character in trace table"); }
  }
  Felt::from(result)
}

fn parse_line(line: String) -> Vec::<Felt> {
  let words = line.split(" ");
  let mut vw = Vec::<Felt>::new();
  for word in words.into_iter() {
     if word != "" {
       vw.push(parse_fast(&word));
     }
   } 
   return vw;
}

fn parse_lines(table_data: String) -> Vec::<Vec::<Felt>> {
  let lines = table_data.split("\n");
  let mut vwords = Vec::<Vec::<Felt>>::new();
  for line in lines.into_iter() {
     if line != "" {
       vwords.push(parse_line(line.to_string()));
     }
  }
  return vwords;
}

fn load_file(f:&str) -> String {
  let mut table_file = File::open (f).unwrap();
  let mut table_data = String::new(); 
  table_file.read_to_string(&mut table_data).unwrap();
  return table_data;
}

fn get_public_values(input_f:&str, output_f:&str) -> PublicInputs {
  let z = Felt::from(1 as u64);
  let inputs = parse_line(load_file(input_f));
  let mut input_a : [Felt;2] = [z,z];
  for (i,v) in inputs.iter().enumerate() {
    input_a[i]=*v; 
  }
  let outputs = parse_line(load_file(output_f));
  let mut output_a : [Felt;2] = [z,z];
  for (i,v) in outputs.iter().enumerate() {
    output_a[i]=*v; 
  }
  PublicInputs::new(input_a, output_a)
}


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
    pub fn build_trace(&self, table_f: &str) -> TraceTable<Felt> {

        let table_data = load_file(table_f);
        let vwords = parse_lines(table_data); 
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
    type Air = FibAir;
    type Trace = TraceTable<Felt>;
    type HashFn = H;
    type RandomCoin = DefaultRandomCoin<Self::HashFn>;

    fn get_pub_inputs(&self, trace: &Self::Trace) -> PublicInputs {
        let mut inputs = [Felt::ONE; 2];
        inputs[0] = trace.get(0, 0); // a
        inputs[1] = trace.get(1, 0); // b

        let mut outputs = [Felt::ONE; 2];
        let last_step = trace.length() - 1; // why is this 2?
        outputs[0] = trace.get(0, last_step); // a
        outputs[1] = trace.get(1, last_step); // b
        PublicInputs::new(inputs, outputs)
    }

    fn options(&self) -> &WinterProofOptions {
        &self.options
    }
}

fn main() {
    let args: Vec<String> = env::args().collect(); 
    let input_f = &args[1];
    let trace_f = &args[2];
    let output_f = &args[3];


    let options = WinterProofOptions::new(27, 8, 16, FieldExtension::None, 8, 255);
    let prover = ExampleProver::<Blake3_192<Felt>>::new(options);
    let trace = prover.build_trace(trace_f);
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
    verify::<FibAir, Blake3_192<Felt>, DefaultRandomCoin<Blake3_192<Felt>>>(proof, pub_inputs)
        .unwrap();
}

