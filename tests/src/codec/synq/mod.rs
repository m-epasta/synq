mod compilation_test;
mod lexer_test;

#[macro_export]
macro_rules! compile {
    ($rel_file:expr) => {
        synq_codec::synq::compile({
            let base = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
            let full_path = base.join(format!("files/{}", $rel_file));
            std::fs::read_to_string(full_path).unwrap()
        })
    };
}
