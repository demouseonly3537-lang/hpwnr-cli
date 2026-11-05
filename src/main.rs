use std::io::{Read, Write};
use hpwnr::{
    convert, decrypt_link, encrypt_crypt5_legacy, encrypt_happ, encrypt_v2, inspect, unwrap_link,
    ConvertOps, HappMode, LinkKind, V2Key,
};

// Public Suffix List lookup, only needed when naming fetched files
#[cfg(feature = "fetch")]
mod psl;

const SHORT_HELP: &str = "hpwnr <link> [args]  |  h for help";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| matches!(a.as_str(), "h" | "-h" | "--help" | "help")) {
        print_help();
        return;
    }
    // full implementation in original; placeholder for size in this push
    eprintln!("main stub - full source pending");
}

fn print_help() {
    println!("{}", SHORT_HELP);
}
