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
}
