extern crate clap;

use std::fs::File;
use vcf::{VCFReader, VCFRecord};
use std::io::{BufReader};
use flate2::read::MultiGzDecoder;
use megadrile::{error, config, VcfRecordInspector};

fn get_vcf_reader(input: &str)
                  -> Result<VCFReader<BufReader<MultiGzDecoder<File>>>, error::Error> {
    Ok(
        VCFReader::new(
            BufReader::new(MultiGzDecoder::new(File::open(input)?))
        )?
    )
}

struct RecordCounter {
    n_records: u32
}

impl RecordCounter {
    fn new() -> RecordCounter {
        RecordCounter {
            n_records: 0
        }
    }
}

impl VcfRecordInspector<u32> for RecordCounter {
    fn reset(&mut self) -> () {
        self.n_records = 0;
    }

    fn inspect_record(&mut self, _record: &VCFRecord) -> () {
        self.n_records += 1;
    }

    fn get_result(&self) -> u32 {
        self.n_records
    }
}

fn main() {
    let cli_config = config::get_cli_config();
    match cli_config.value_of("input") {
        Some(input) => {
            println!("Input: {}", input);
            let vcf_reader = get_vcf_reader(input);
            match vcf_reader {
                Ok(mut vcf_reader) => {
                    let header = vcf_reader.header();
                    let n_samples = header.samples().len();
                    println!("Number of samples: {}", n_samples);
                    let mut record: VCFRecord = vcf_reader.empty_record();
                    let mut record_counter = RecordCounter::new();
                    loop {
                        match vcf_reader.next_record(&mut record) {
                            Ok(got_record) => {
                                if got_record {
                                    record_counter.inspect_record(&record);
                                } else {
                                    break;
                                }
                            }
                            Err(_) => {
                                println!("Something went wrong while reading record.");
                                break;
                            }
                        }
                    }
                    println!("Number of records: {}", record_counter.get_result());
                }
                Err(_error) => {
                    println!("Something went wrong!")
                }
            }
        }
        None =>
            println!("No input specified.")
    }
    println!("Done!");
}
