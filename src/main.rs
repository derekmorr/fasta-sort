extern crate rayon;
extern crate seq_io;

use std::env;
use std::str;
use seq_io::fasta::{Reader, Record};
use rayon::prelude::*;

struct SeqData {
    head: Vec<u8>,
    seqlines: Vec<u8>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut reader = Reader::from_path(filename).expect("Error opening file");


    // read the header and raw sequence bytes (including newlines) into 'records'
    let mut records: Vec<SeqData> = Vec::new();
    while let Some(record) = reader.next() {
        let record = record.expect("Error reading record");
        let s = SeqData {
            head: record.head().to_vec(),
            seqlines: record.seq().to_vec(),
        };
        records.push(s);
    }

    // do a parallel sort of the records
    records.par_sort_by_key(|r| r.seqlines.len());

    // write all of the records to stdout
    for record in records {
        println!(">{}", str::from_utf8(&record.head).unwrap());
        println!("{}", str::from_utf8(&record.seqlines).unwrap());
    }
}
