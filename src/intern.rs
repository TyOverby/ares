use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub struct Symbol(u32);

#[derive(Debug)]
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
        let ret = Symbol(self.current_id);
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

    pub fn gen_sym_prefix<S: AsRef<str> + Into<String>>(&mut self, prefix: S) -> Symbol {
        let sym = self.gen_sym();
        let Symbol(id) = sym;
        let sym_str = format!("{}{}", prefix.as_ref(), id);
        self.sym_to_string.insert(sym, sym_str.clone());
        self.string_to_sym.insert(sym_str, sym);
        sym
    }

    pub fn symbol_for_name<S: ?Sized + AsRef<str>>(&self, symbol_str: &S) -> Option<Symbol> {
        self.string_to_sym.get(symbol_str.as_ref()).cloned()
    }

    pub fn contains<S: AsRef<str>>(&self, symbol_str: S) -> bool {
        self.string_to_sym.contains_key(symbol_str.as_ref())
    }

    pub fn lookup(&self, symbol: Symbol) -> Option<&str> {
        self.sym_to_string.get(&symbol).map(|s| &s[..])
    }

    pub fn lookup_or_anon(&self, symbol: Symbol) -> String {
        let Symbol(id) = symbol;
        self.lookup(symbol)
            .map(|s| s.into())
            .unwrap_or_else(|| format!("s{}", id))
    }
}
