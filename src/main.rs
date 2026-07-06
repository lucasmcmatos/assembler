mod code;
mod parser;
mod symbol_table;

use parser::{InstructionType, Parser};
use std::env;
use std::fs;
use std::io;
use std::process;
use symbol_table::SymbolTable;

fn first_pass(filename: &str, symbol_table: &mut SymbolTable) -> io::Result<()> {
    let mut parser = Parser::new(filename)?;
    let mut address: u16 = 0;

    while parser.has_more_instructions() {
        let line = parser.advance();
        if Parser::instruction_type(line) == InstructionType::Label {
            let label = Parser::symbol(line);
            symbol_table.add_entry(&label, address);
        } else {
            address += 1;
        }
    }

    Ok(())
}

fn resolve_a_instruction_address(symbol: &str, symbol_table: &mut SymbolTable) -> u16 {
    if let Ok(value) = symbol.parse::<u16>() {
        return value;
    }
    if let Some(address) = symbol_table.get_address(symbol) {
        return address;
    }
    symbol_table.add_variable(symbol)
}

fn encode_c_instruction(line: &str) -> String {
    let dest = code::dest(Parser::dest(line));
    let comp = code::comp(Parser::comp(line));
    let jump = code::jump(Parser::jump(line));
    format!("111{comp}{dest}{jump}")
}

fn second_pass(filename: &str, symbol_table: &mut SymbolTable) -> io::Result<Vec<String>> {
    let mut parser = Parser::new(filename)?;
    let mut binary_lines = Vec::new();

    while parser.has_more_instructions() {
        let line = parser.advance();
        match Parser::instruction_type(line) {
            InstructionType::AInstruction => {
                let symbol = Parser::symbol(line);
                let address = resolve_a_instruction_address(&symbol, symbol_table);
                binary_lines.push(format!("{address:016b}"));
            }
            InstructionType::CInstruction => {
                binary_lines.push(encode_c_instruction(line));
            }
            InstructionType::Label => {}
        }
    }

    Ok(binary_lines)
}

fn write_hack_file(filename: &str, binary_lines: &[String]) -> io::Result<()> {
    let output_path = std::path::Path::new(filename).with_extension("hack");
    let mut contents = binary_lines.join("\n");
    contents.push('\n');
    fs::write(output_path, contents)
}

fn main() {
    let filename = match env::args().nth(1) {
        Some(f) => f,
        None => {
            eprintln!("uso: assembler <arquivo.asm>");
            process::exit(1);
        }
    };

    let mut symbol_table = SymbolTable::new();
    if let Err(err) = first_pass(&filename, &mut symbol_table) {
        eprintln!("erro ao ler {filename}: {err}");
        process::exit(1);
    }

    let binary_lines = match second_pass(&filename, &mut symbol_table) {
        Ok(lines) => lines,
        Err(err) => {
            eprintln!("erro ao ler {filename}: {err}");
            process::exit(1);
        }
    };

    if let Err(err) = write_hack_file(&filename, &binary_lines) {
        eprintln!("erro ao escrever saída de {filename}: {err}");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn write_temp_asm(contents: &str) -> std::path::PathBuf {
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let mut path = std::env::temp_dir();
        path.push(format!(
            "assembler_main_test_{}_{id}.asm",
            std::process::id()
        ));
        fs::write(&path, contents).expect("failed to write temp .asm file");
        path
    }

    #[test]
    fn first_pass_registers_labels_with_address_of_next_real_instruction() {
        let path = write_temp_asm(
            "(LOOP)\n\
             @1\n\
             D=A\n\
             (END)\n\
             @2\n",
        );

        let mut symbol_table = SymbolTable::new();
        first_pass(path.to_str().unwrap(), &mut symbol_table).unwrap();

        assert_eq!(symbol_table.get_address("LOOP"), Some(0));
        assert_eq!(symbol_table.get_address("END"), Some(2));

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn first_pass_does_not_register_a_or_c_instructions_as_labels() {
        let path = write_temp_asm("@1\nD=A\n");

        let mut symbol_table = SymbolTable::new();
        first_pass(path.to_str().unwrap(), &mut symbol_table).unwrap();

        assert!(!symbol_table.contains("1"));
        assert!(!symbol_table.contains("D=A"));

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn unreferenced_label_is_registered_but_silently_unused() {
        let path = write_temp_asm("(UNUSED)\n@1\nD=A\n");

        let mut symbol_table = SymbolTable::new();
        first_pass(path.to_str().unwrap(), &mut symbol_table).unwrap();
        let binary_lines = second_pass(path.to_str().unwrap(), &mut symbol_table).unwrap();

        assert_eq!(binary_lines, vec!["0000000000000001", "1110110000010000"]);
        assert_eq!(symbol_table.get_address("UNUSED"), Some(0));

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn resolves_numeric_constant_address() {
        let mut symbol_table = SymbolTable::new();
        assert_eq!(resolve_a_instruction_address("123", &mut symbol_table), 123);
    }

    #[test]
    fn resolves_predefined_symbol_address() {
        let mut symbol_table = SymbolTable::new();
        assert_eq!(
            resolve_a_instruction_address("SCREEN", &mut symbol_table),
            16384
        );
    }

    #[test]
    fn allocates_new_variables_sequentially_from_16() {
        let mut symbol_table = SymbolTable::new();
        assert_eq!(resolve_a_instruction_address("i", &mut symbol_table), 16);
        assert_eq!(resolve_a_instruction_address("j", &mut symbol_table), 17);
    }

    #[test]
    fn reuses_address_for_already_allocated_variable() {
        let mut symbol_table = SymbolTable::new();
        let first = resolve_a_instruction_address("i", &mut symbol_table);
        let second = resolve_a_instruction_address("i", &mut symbol_table);
        assert_eq!(first, second);
    }

    #[test]
    fn second_pass_resolves_constants_and_known_symbols_for_a_instructions() {
        let path = write_temp_asm("(LOOP)\n@0\n@SCREEN\n@LOOP\n");

        let mut symbol_table = SymbolTable::new();
        first_pass(path.to_str().unwrap(), &mut symbol_table).unwrap();
        let binary_lines = second_pass(path.to_str().unwrap(), &mut symbol_table).unwrap();

        assert_eq!(
            binary_lines,
            vec!["0000000000000000", "0100000000000000", "0000000000000000",]
        );

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn encodes_c_instruction_with_dest_comp_and_jump() {
        assert_eq!(encode_c_instruction("D=D+1;JGT"), "1110011111010001");
    }

    #[test]
    fn encodes_c_instruction_without_jump() {
        assert_eq!(encode_c_instruction("M=D"), "1110001100001000");
    }

    #[test]
    fn encodes_c_instruction_without_dest() {
        assert_eq!(encode_c_instruction("0;JMP"), "1110101010000111");
    }

    #[test]
    fn second_pass_encodes_a_and_c_instructions_together() {
        let path = write_temp_asm("@0\nD=A\n@1\nD=D+M\n@2\nM=D\n");

        let mut symbol_table = SymbolTable::new();
        first_pass(path.to_str().unwrap(), &mut symbol_table).unwrap();
        let binary_lines = second_pass(path.to_str().unwrap(), &mut symbol_table).unwrap();

        assert_eq!(
            binary_lines,
            vec![
                "0000000000000000",
                "1110110000010000",
                "0000000000000001",
                "1111000010010000",
                "0000000000000010",
                "1110001100001000",
            ]
        );

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn write_hack_file_creates_file_with_binary_lines() {
        let asm_path = write_temp_asm("@0\nD=A\n");
        let binary_lines = vec![
            "0000000000000000".to_string(),
            "1110110000010000".to_string(),
        ];

        write_hack_file(asm_path.to_str().unwrap(), &binary_lines).unwrap();

        let hack_path = asm_path.with_extension("hack");
        let contents = fs::read_to_string(&hack_path).unwrap();
        assert_eq!(contents, "0000000000000000\n1110110000010000\n");

        fs::remove_file(asm_path).unwrap();
        fs::remove_file(hack_path).unwrap();
    }

    #[test]
    fn second_pass_allocates_new_variables_starting_at_16() {
        let path = write_temp_asm("@i\n@j\n@i\n");

        let mut symbol_table = SymbolTable::new();
        first_pass(path.to_str().unwrap(), &mut symbol_table).unwrap();
        let binary_lines = second_pass(path.to_str().unwrap(), &mut symbol_table).unwrap();

        assert_eq!(
            binary_lines,
            vec!["0000000000010000", "0000000000010001", "0000000000010000",]
        );
        assert_eq!(symbol_table.get_address("i"), Some(16));
        assert_eq!(symbol_table.get_address("j"), Some(17));

        fs::remove_file(path).unwrap();
    }
}
