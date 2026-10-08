use indexmap::IndexSet;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol(u32);

impl Symbol {
    pub fn intern(interner: &SymbolInterner, s: &str) -> Self {
        interner.intern(s)
    }

    pub fn resolve(self, interner: &SymbolInterner) -> Arc<str> {
        interner.resolve(self)
    }

    fn from_usize(n: usize) -> Self {
        Self(n as u32)
    }

    fn to_usize(self) -> usize {
        self.0 as usize
    }
}

#[derive(Debug)]
pub struct SymbolInterner {
    inner: RwLock<SymbolInternerImpl>,
}

impl SymbolInterner {
    pub fn new() -> Self {
        let inner = RwLock::new(SymbolInternerImpl::new());
        Self { inner }
    }

    pub fn intern(&self, s: &str) -> Symbol {
        if let Some(idx) = self
            .inner
            .read()
            .expect("unable to acquire read lock")
            .symbols
            .get_index_of(s)
        {
            return Symbol::from_usize(idx);
        }

        let mut inner = self.inner.write().expect("unable to acquire write lock");
        if let Some(idx) = inner.symbols.get_index_of(s) {
            // Somebody added file while we were waiting for write lock
            return Symbol::from_usize(idx);
        }

        let (idx, _) = inner.symbols.insert_full(s.into());
        Symbol::from_usize(idx)
    }

    pub fn resolve(&self, sym: Symbol) -> Arc<str> {
        let inner = self.inner.read().expect("unable to acquire read lock");
        Arc::clone(&inner.symbols[sym.to_usize()])
    }
}

impl Default for SymbolInterner {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct SymbolInternerImpl {
    symbols: IndexSet<Arc<str>>,
}

impl SymbolInternerImpl {
    pub fn new() -> Self {
        Self {
            symbols: IndexSet::new(),
        }
    }
}

impl Default for SymbolInternerImpl {
    fn default() -> Self {
        Self::new()
    }
}
