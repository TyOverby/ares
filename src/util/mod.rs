pub use self::rc_slice::RcSlice;
use std::io::{self, BufRead, Write};
use std::hash::Hasher;

pub fn prompt<P: ?Sized + AsRef<str>>(prompt: &P) -> Option<String> {
    let printed = write!(io::stdout(), "{}", prompt.as_ref());
    let flushed = io::stdout().flush();

    let mut buffer = String::new();
    let read = io::stdin().read_line(&mut buffer);

    match (printed, flushed, read) {
        (Ok(_), Ok(_), Ok(0)) => None,
        (Ok(_), Ok(_), Ok(_)) => Some(buffer),
        _ => None,
    }
}

/// A hasher implementation made specifically for
/// hashing u32 key values.
///
/// This is *NOT* a cryptographic hash.
pub struct IdentityHash {
    state: u32
}

impl IdentityHash {
    pub fn new() -> IdentityHash {
        IdentityHash { state: 0 }
    }
}

impl Default for IdentityHash {
    fn default() -> IdentityHash {
        IdentityHash::new()
    }
}

impl Hasher for IdentityHash {
    fn finish(&self) -> u64 {
        self.state as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state <<= 8;
            self.state += byte as u32;
        }
    }

    fn write_u32(&mut self, v: u32) {
        self.state = v;
    }
}


pub mod rc_slice {
    #![allow(unused)]
    use std::rc::Rc;
    use {Value, rc_to_usize};

    #[derive(Clone)]
    pub struct RcSlice {
        data: Rc<Vec<Value>>,
        start: usize,
        len: usize,
    }

    impl RcSlice {
        pub fn empty() -> RcSlice {
            RcSlice::new(vec![])
        }

        pub fn new(v: Vec<Value>) -> RcSlice {
            let len = v.len();
            RcSlice {
                data: Rc::new(v),
                start: 0,
                len: len,
            }
        }

        pub fn tail(&self) -> RcSlice {
            self.slice(1, self.len)
        }

        pub fn init(&self) -> RcSlice {
            self.slice(0, self.len - 1)
        }

        pub fn len(&self) -> usize {
            self.len
        }

        pub fn is_empty(&self) -> bool {
            self.len == 0
        }

        pub fn slice(&self, from: usize, to: usize) -> RcSlice {
            // TODO: make this checked subtractions
            let len = to - from;
            RcSlice {
                data: self.data.clone(),
                start: from,
                len: len,
            }
        }

        pub fn get_slice(&self) -> &[Value] {
            &self.data[self.start..self.start + self.len]
        }

        pub fn add(&self, v: Value) -> RcSlice {
            let mut data: Vec<_> = (*self.data).clone();
            data.push(v);
            RcSlice::new(data)
        }

        pub fn add_all<I>(&self, i: I) -> RcSlice
            where I: Iterator<Item = Value>
        {
            let mut data: Vec<_> = (*self.data).clone();
            data.extend(i);
            RcSlice::new(data)
        }
    }

    impl PartialEq for RcSlice {
        fn eq(&self, other: &RcSlice) -> bool {
            // lengths *have* to be equal
            self.len == other.len &&
            ((rc_to_usize(&self.data) == rc_to_usize(&self.data) && self.start == other.start) ||
             self.get_slice() == other.get_slice())
        }
    }

    impl Eq for RcSlice {}

    impl ::std::hash::Hash for RcSlice {
        fn hash<H>(&self, state: &mut H)
            where H: ::std::hash::Hasher
        {
            self.get_slice().hash(state)
        }
    }
}
