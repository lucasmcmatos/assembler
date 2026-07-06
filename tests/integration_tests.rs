use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

// Endereços de ITSR0=10, OUTPUT_D=12 e END=14 foram conferidos manualmente
// contra o programa Max.asm/MaxL.asm de referência do Nand2Tetris.
const ADD_HACK: &[&str] = &[
    "0000000000000010",
    "1110110000010000",
    "0000000000000011",
    "1110000010010000",
    "0000000000000000",
    "1110001100001000",
];

const MAX_HACK: &[&str] = &[
    "0000000000000000",
    "1111110000010000",
    "0000000000000001",
    "1111010011010000",
    "0000000000001010",
    "1110001100000001",
    "0000000000000001",
    "1111110000010000",
    "0000000000001100",
    "1110101010000111",
    "0000000000000000",
    "1111110000010000",
    "0000000000000010",
    "1110001100001000",
    "0000000000001110",
    "1110101010000111",
];

// Trace manual: R0=0, SCREEN=16384, n=16 (variável), addr=17 (variável),
// LOOP=10, END=23. Conferido instrução a instrução contra rect.asm.
const RECT_HACK: &[&str] = &[
    "0000000000000000",
    "1111110000010000",
    "0000000000010111",
    "1110001100000110",
    "0000000000010000",
    "1110001100001000",
    "0100000000000000",
    "1110110000010000",
    "0000000000010001",
    "1110001100001000",
    "0000000000010001",
    "1111110000100000",
    "1110111010001000",
    "0000000000010001",
    "1111110000010000",
    "0000000000100000",
    "1110000010010000",
    "0000000000010001",
    "1110001100001000",
    "0000000000010000",
    "1111110010011000",
    "0000000000001010",
    "1110001100000001",
    "0000000000010111",
    "1110101010000111",
];

fn run_assembler_on(source_path: &str) -> Vec<String> {
    let id = COUNTER.fetch_add(1, Ordering::SeqCst);
    let mut asm_path: PathBuf = std::env::temp_dir();
    asm_path.push(format!(
        "assembler_integration_{}_{id}.asm",
        std::process::id()
    ));
    fs::copy(source_path, &asm_path).expect("failed to copy source .asm to temp dir");

    let status = Command::new(env!("CARGO_BIN_EXE_assembler"))
        .arg(&asm_path)
        .status()
        .expect("failed to run assembler binary");
    assert!(status.success(), "assembler failed for {source_path}");

    let hack_path = asm_path.with_extension("hack");
    let contents = fs::read_to_string(&hack_path).expect("failed to read generated .hack file");

    fs::remove_file(&asm_path).unwrap();
    fs::remove_file(&hack_path).unwrap();

    contents.lines().map(str::to_string).collect()
}

#[test]
fn add_asm_matches_reference_hack() {
    let generated = run_assembler_on("test_files/add.asm");
    assert_eq!(generated, ADD_HACK);
}

#[test]
fn max_asm_matches_reference_hack() {
    let generated = run_assembler_on("test_files/max.asm");
    assert_eq!(generated, MAX_HACK);
}

#[test]
fn maxl_asm_matches_reference_hack() {
    let generated = run_assembler_on("test_files/maxL.asm");
    assert_eq!(generated, MAX_HACK);
}

#[test]
fn rect_asm_matches_reference_hack() {
    let generated = run_assembler_on("test_files/rect.asm");
    assert_eq!(generated, RECT_HACK);
}

#[test]
fn rectl_asm_matches_reference_hack() {
    let generated = run_assembler_on("test_files/rectL.asm");
    assert_eq!(generated, RECT_HACK);
}
