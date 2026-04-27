use std::path::PathBuf;
use synq_codec::synq::compile;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    unsafe { std::env::set_var("PHOTON_PRINT_AST", "1") };

    if args.len() == 2 {
        let path = PathBuf::from(&args[1]);
        println!("{path:?}");

        let content = match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error reading '{}': {}", path.display(), e);
                std::process::exit(1);
            }
        };

        if let Err(e) = compile(content.clone()) {
            eprintln!("{}", e.pretty_print(&content));
            std::process::exit(1);
        }
    }
}
