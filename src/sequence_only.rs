te rayon;
extern crate seq_io;

use std::env;
use std::str;
use seq_io::fasta::{Reader, Record};
use rayon::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut reader = Reader::from_path(filename).expect("Error opening file");

    // read the raw sequence bytes (including newlines) into 'records'
    let mut records: Vec<Vec<u8>> = Vec::new();
    while let Some(record) = reader.next() {
        let record = record.expect("Error reading record");
        records.push(record.seq().to_vec());
    }

    // do a parallel sort of records
    records.par_sort_by_key(|r| r.len());

    // write all of the records to stdout
    for record in records {
        println!("{}", str::from_utf8(&record).unwrap());
    }

}

