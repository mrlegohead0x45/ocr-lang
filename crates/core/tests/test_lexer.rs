use std::fs::File;
use std::path::Path;

use paste::paste;

use core::Lexer;

macro_rules! make_test {
    ($name: expr) => {
        paste! {
            #[test]
            fn [< test_ $name >] (){
                let source_filename = concat!(stringify!($name), ".ocr");
                // e.g .../ocr-lang/crates/core/tests/data/lexer/filename.ocr
                // .join()s needed for windows compatibility, as that knows whether to use / or \
                let source_file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("tests")
                    .join("data")
                    .join("lexer")
                    .join(source_filename);

                let mut source_file = File::open(&source_file_path).unwrap_or_else(|_| panic!("Could not open {}", source_file_path.display()));
                let lexer = Lexer::new(Box::new(source_file), source_filename.to_string());

            }
        }
    };
}

make_test!(hello_world);
