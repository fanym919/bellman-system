//#![allow(unused_imports)]
#![allow(unused_variables)]
extern crate bellman;
extern crate pairing;
extern crate rand;
use bellman::{groth16::{Parameters}};
use pairing::{PrimeField};
use std::{fs, env, io::{Error}};

extern crate rustc_serialize;
use rustc_serialize::json::Json;
use std::fs::File;
use std::io::Read;

mod cube;

fn main() -> Result<(), Error> {
    use pairing::bls12_381::{Bls12, Fr};
    use rand::thread_rng;
    use bellman::groth16::{
        create_random_proof
    };

    let args: Vec<_> = env::args().collect();
    let pk_path: &str = &args[1].to_owned()[..];
    let proof_path: &str = &args[2].to_owned()[..];
    let witness_path: &str = &args[3].to_owned()[..];
    
    let rng = &mut thread_rng();
    
    // Read proving key
    let mut pk_f = fs::File::open(pk_path)?;
    let pk = Parameters::<Bls12>::read(&mut pk_f, true).unwrap();

    println!("Creating proofs...");

    // Read Witness (private variables only)
    let mut file = File::open(witness_path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let json = Json::from_str(&data).unwrap();
    let witness = json.as_object().unwrap();
    
    // Create an instance of circuit
    let c = cube::CubeDemo::<Bls12> {
        x: Fr::from_str(format!("{}", witness.get("x").unwrap()).as_str()),
        tmp_1: Fr::from_str(format!("{}", witness.get("tmp_1").unwrap()).as_str()),
        y: Fr::from_str(format!("{}", witness.get("y").unwrap()).as_str()),
        tmp_2: Fr::from_str(format!("{}", witness.get("tmp_2").unwrap()).as_str()),
        out: Fr::from_str(format!("{}", witness.get("out").unwrap()).as_str())
    };
    
    // Create a groth16 proof with our parameters.
    let proof = create_random_proof(c, &pk, rng).unwrap();
    
    // Write proof
    let mut w = vec![];
    proof.write(&mut w)?;
    fs::write(proof_path, &w[..])?;

    Ok(())
}