extern crate rayon;
extern crate seq_io;

use std::env;
use std::io;
use seq_io::fasta::{Reader, Record, OwnedRecord, write_to};
use rayon::prelude::*;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut reader = Reader::from_path(filename).expect("Error opening file");

    let records: Result<Vec<_>, _> = reader.records().collect();
    let mut j: Vec<OwnedRecord> = records.unwrap();
    j.par_sort_by_key(|r| r.seq().len());
    
    let mut stdout = io::stdout();
    for record in j.iter_mut() {
        write_to(&mut stdout, record.head(), record.seq());
    }
}
