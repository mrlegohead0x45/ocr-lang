# ocr-lang

ocr-lang is an interpreter for OCR Exam Reference Language, the psuedocode
found in OCR Computer Science exams. Examples can be found in the
[`examples` directory](https://github.com/mrlegohead0x45/ocr-lang/tree/main/examples).

## Specifications

The difference specifications can be found here:

- [GCSE](https://ocr.org.uk/Images/558027-specification-gcse-computer-science-j277.pdf) section 3c, page 27
- [AS](https://ocr.org.uk/Images/170845-specification-accredited-as-level-gce-computer-science-h046.pdf) section 5d, page 25
- [A Level](https://ocr.org.uk/Images/170844-specification-accredited-a-level-gce-computer-science-h446.pdf) section 5d, page 37

They represent the different concepts that appear in different levels.
A Level builds on AS which builds on GCSE. The main differences are that AS
introduces pass by value and reference, and A Level introduces OOP.

## Installation

### From source

[Install Rust](https://www.rust-lang.org/tools/install), clone the repo or download the source from
[the releases page](https://github.com/mrlegohead0x45/ocr-lang/releases) and run `cargo build -r`.
The executable will be in `target/release`

### Prebuilt binary

If you're on Linux or Windows, download a prebuilt executable from
[the releases page](https://github.com/mrlegohead0x45/ocr-lang/releases)

## PRs

Pull Requests will not be accepted, because this might end up being my NEA programming project,
in which case it would need to be entirely my own work.

## Plans

I plan to also write a compiler using LLVM. I may also add a GUI using
[egui](https://github.com/emilk/egui). This would also run in the browser using WebAssembly.
