use paste::paste;
use std::fs::File;

macro_rules! make_test {
    ($name: expr) => {
        paste! {
            #[test]
            fn [< test_ $name >] (){
                let source_filename = concat!(stringify!($name), ".ocr");
                let mut source_file = File::open(source_filename).unwrap_or_else(|_| panic!("Could not open {}", source_filename));
                let lexer = core::Lexer::new(Box::new(source_file), source_filename.to_string());

            }
        }
    };
}

// make_test!(hello_world);
