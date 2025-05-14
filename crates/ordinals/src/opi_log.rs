use std::fs::File;
use std::io::Write;
use std::sync::Mutex;
use lazy_static::lazy_static;

pub fn log_to_file(chain: &str, to_write: String, flush: bool, height: u32, first_in_block: &mut bool) {
    lazy_static! {
      static ref RUNES_OUTPUT: Mutex<Option<File>> = Mutex::new(None);
    }
    let mut runes_output = RUNES_OUTPUT.lock().unwrap();
    if runes_output.as_ref().is_none() {
        let chain_folder: String = match chain {
            "mainnet" => String::from(""),
            "regtest" => String::from("/regtest/"),
            "signet" => String::from("/signet/"),
            "testnet" => String::from("/testnet3/"),
            "testnet4" => String::from("/testnet4/"),
            _ => String::from("")
        };
        *runes_output = Some(File::options().append(true).open(format!("{chain_folder}runes_output.txt")).unwrap());
    }
    if to_write != "" {
        if *first_in_block {
            println!("cmd;{0};block_start", height,);
            writeln!(runes_output.as_ref().unwrap(), "cmd;{0};block_start", height, ).expect("log_to_file failed.");
        }
        *first_in_block = false;

        writeln!(runes_output.as_ref().unwrap(), "{}", to_write).expect("log_to_file failed.");
    }
    if flush {
        (runes_output.as_ref().unwrap()).flush().expect("log to file flush failed");
    }
}


pub const MAINNET: &str = "mainnet";
pub const REGTEST: &str = "regtest";
pub const SIGNET: &str = "signet";
pub const TESTNET: &str = "testnet";
pub const TESTNET4: &str = "testnet4";