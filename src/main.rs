// TODO: remover allow(dead_code) quando main.rs consumir o restante da API (etapas 9-11 do roteiro)
#[allow(dead_code)]
mod code;
#[allow(dead_code)]
mod parser;
#[allow(dead_code)]
mod symbol_table;

use parser::{InstructionType, Parser};
use std::env;
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

fn resolve_a_instruction_address(symbol: &str, symbol_table: &SymbolTable) -> u16 {
    if let Ok(value) = symbol.parse::<u16>() {
        return value;
    }
    symbol_table.get_address(symbol).unwrap_or_else(|| {
        panic!("símbolo '{symbol}' ainda não suportado (alocação de variáveis vem na etapa 10)")
    })
}

fn second_pass(filename: &str, symbol_table: &mut SymbolTable) -> io::Result<Vec<String>> {
    let mut parser = Parser::new(filename)?;
    let mut binary_lines = Vec::new();

    while parser.has_more_instructions() {
        let line = parser.advance();
        if Parser::instruction_type(line) == InstructionType::AInstruction {
            let symbol = Parser::symbol(line);
            let address = resolve_a_instruction_address(&symbol, symbol_table);
            binary_lines.push(format!("{address:016b}"));
        }
        // C-instructions: implementado na etapa 11
    }

    Ok(binary_lines)
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

    if let Err(err) = second_pass(&filename, &mut symbol_table) {
        eprintln!("erro ao ler {filename}: {err}");
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
    fn resolves_numeric_constant_address() {
        let symbol_table = SymbolTable::new();
        assert_eq!(resolve_a_instruction_address("123", &symbol_table), 123);
    }

    #[test]
    fn resolves_predefined_symbol_address() {
        let symbol_table = SymbolTable::new();
        assert_eq!(
            resolve_a_instruction_address("SCREEN", &symbol_table),
            16384
        );
    }

    #[test]
    #[should_panic(expected = "ainda não suportado")]
    fn panics_on_symbol_that_requires_variable_allocation() {
        let symbol_table = SymbolTable::new();
        resolve_a_instruction_address("i", &symbol_table);
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
}
