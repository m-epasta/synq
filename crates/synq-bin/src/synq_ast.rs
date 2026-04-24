#![no_std]
extern crate alloc;
use alloc::string::ToString;
use synq_codec::synq::compile;

fn main() {
    let input = "
@over NATS
@codec minimal
frame Block {
  sender   u64
  receiver u64
  amount   f64
";
    if let Err(e) = compile(input.to_string()) {
        print_no_std::println!("{}", e.pretty_print(input));
    }
}
