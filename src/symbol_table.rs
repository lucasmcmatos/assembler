use std::collections::HashMap;

pub struct SymbolTable {
    table: HashMap<String, u16>,
    next_address: u16,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut table = HashMap::new();

        for i in 0..16 {
            table.insert(format!("R{i}"), i as u16);
        }
        table.insert("SP".to_string(), 0);
        table.insert("LCL".to_string(), 1);
        table.insert("ARG".to_string(), 2);
        table.insert("THIS".to_string(), 3);
        table.insert("THAT".to_string(), 4);
        table.insert("SCREEN".to_string(), 16384);
        table.insert("KBD".to_string(), 24576);

        SymbolTable {
            table,
            next_address: 16,
        }
    }

    pub fn add_entry(&mut self, symbol: &str, address: u16) {
        self.table.insert(symbol.to_string(), address);
    }

    pub fn add_variable(&mut self, symbol: &str) -> u16 {
        if let Some(&address) = self.table.get(symbol) {
            return address;
        }
        let address = self.next_address;
        self.table.insert(symbol.to_string(), address);
        self.next_address += 1;
        address
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.table.contains_key(symbol)
    }

    pub fn get_address(&self, symbol: &str) -> Option<u16> {
        self.table.get(symbol).copied()
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn predefined_symbols_have_correct_addresses() {
        let table = SymbolTable::new();
        assert_eq!(table.get_address("SP"), Some(0));
        assert_eq!(table.get_address("LCL"), Some(1));
        assert_eq!(table.get_address("ARG"), Some(2));
        assert_eq!(table.get_address("THIS"), Some(3));
        assert_eq!(table.get_address("THAT"), Some(4));
        assert_eq!(table.get_address("SCREEN"), Some(16384));
        assert_eq!(table.get_address("KBD"), Some(24576));
    }

    #[test]
    fn predefined_registers_r0_to_r15() {
        let table = SymbolTable::new();
        for i in 0..16 {
            assert_eq!(table.get_address(&format!("R{i}")), Some(i as u16));
        }
    }

    #[test]
    fn unknown_symbol_is_not_contained() {
        let table = SymbolTable::new();
        assert!(!table.contains("FOO"));
        assert_eq!(table.get_address("FOO"), None);
    }

    #[test]
    fn add_entry_registers_label_address() {
        let mut table = SymbolTable::new();
        table.add_entry("LOOP", 4);
        assert!(table.contains("LOOP"));
        assert_eq!(table.get_address("LOOP"), Some(4));
    }

    #[test]
    fn add_variable_allocates_sequentially_from_16() {
        let mut table = SymbolTable::new();
        let i = table.add_variable("i");
        let j = table.add_variable("j");
        assert_eq!(i, 16);
        assert_eq!(j, 17);
    }

    #[test]
    fn add_variable_is_idempotent_for_existing_symbol() {
        let mut table = SymbolTable::new();
        let first = table.add_variable("i");
        let second = table.add_variable("i");
        assert_eq!(first, second);
    }
}
