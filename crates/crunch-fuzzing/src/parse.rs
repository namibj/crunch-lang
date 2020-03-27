use crunch_parser::{files::FileId, Interner};
use honggfuzz::fuzz;

fn main() {
    let interner = Interner::new();

    loop {
        fuzz!(|bytes: &[u8]| {
            if let Ok(input_str) = std::str::from_utf8(bytes) {
                let _ = crunch_parser::Parser::new(&input_str, FileId::new(0), interner.clone())
                    .parse();
            }
        });
    }
}
