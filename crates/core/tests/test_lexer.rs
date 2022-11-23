use std::fs::File;
use std::io::Read;
use std::path::Path;

use paste::paste;
use ron::from_str;

use core::{Error, Lexer, Token};

macro_rules! make_test {
    ($name: expr) => {
        paste! {
            #[test]
            fn [< test_ $name >] (){
                let source_filename = concat!(stringify!($name), ".ocr");
                let expected_filename = concat!(stringify!($name), ".ron");

                // e.g .../ocr-lang/crates/core/tests/data/lexer/
                // .join()s needed for windows compatibility, as that knows whether to use / or \
                let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("tests")
                    .join("data")
                    .join("lexer");

                // open the files
                let source_file = File::open(dir.join(source_filename))
                    .unwrap_or_else(|_| panic!("Could not open {}", dir.join(source_filename).display()));
                let mut expected_file = File::open(dir.join(expected_filename))
                    .unwrap_or_else(|_| panic!("Could not open {}", dir.join(expected_filename).display()));


                // get the expected result
                let mut buf = String::new();
                expected_file.read_to_string(&mut buf)
                    .unwrap_or_else(|_| panic!("Could not read {}", dir.join(expected_filename).display()));

                let expected: Result<Vec<Token>, Error> = from_str(&buf).expect("Could not deserialise the expected result");

                // lex the source
                let mut lexer = Lexer::new(Box::new(source_file), source_filename.to_string());
                let mut res = lexer.make_tokens();

                // If the result is an error, we cannot rely on the error messages being equal,
                // so set it to be empty, which is what it will be in the expected result
                if let Err(e) = res {
                    res = Err(Error {
                        kind: e.kind,
                        msg: String::new(),
                        pos: e.pos,
                    })
                }

                assert_eq!(res, expected);
            }
        }
    };
}

// make_test!(hello_world);
