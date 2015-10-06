use std::collections::HashMap;

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub struct Symbol {
    id: u32
}

pub struct SymbolIntern {
    current_id: u32,
    sym_to_string: HashMap<Symbol, String>,
    string_to_sym: HashMap<String, Symbol>,
}

impl SymbolIntern {
    pub fn new() -> SymbolIntern {
        SymbolIntern {
            current_id: 0,
            sym_to_string: HashMap::new(),
            string_to_sym: HashMap::new(),
        }
    }

    pub fn gen_sym(&mut self) -> Symbol {
        let ret = Symbol { id: self.current_id };
        self.current_id += 1;
        ret
    }

    pub fn intern<S: AsRef<str> + Into<String>>(&mut self, symbol_str: S) -> Symbol {
        if self.string_to_sym.contains_key(symbol_str.as_ref()) {
            self.string_to_sym[symbol_str.as_ref()]
        } else {
            let symbol_str = symbol_str.into();
            let symbol = self.gen_sym();
            self.sym_to_string.insert(symbol, symbol_str.clone());
            self.string_to_sym.insert(symbol_str, symbol);
            symbol
        }
    }

    pub fn lookup(&self, symbol: &Symbol) -> Option<&str> {
        self.sym_to_string.get(symbol).map(|s| &s[..])
    }
}

