pub use self::rc_slice::RcSlice;

pub mod rc_slice {
    use std::rc::Rc;
    use ::{Value, rc_to_usize};

    #[derive(Clone)]
    pub struct RcSlice {
        data: Rc<Vec<Value>>,
        start: usize,
        len: usize
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
                len: len
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
                len: len
            }
        }

        pub fn get_slice(&self) -> &[Value] {
            &self.data[self.start .. self.start + self.len]
        }

        pub fn add(&self, v: Value) -> RcSlice {
            let mut data: Vec<_> = (*self.data).clone();
            data.push(v);
            RcSlice::new(data)
        }

        pub fn add_all<I>(&self, i: I) -> RcSlice
        where I: Iterator<Item=Value>
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
                (
                    // shortcut
                    (
                        // if they are pointing to the same thing
                        rc_to_usize(&self.data) == rc_to_usize(&self.data) &&
                        // and they start at the same spot, then we can short cut
                        // large equality checks
                        self.start == other.start
                    ) ||
                    // Do expensive compute
                    self.get_slice() == other.get_slice()
                )
        }
    }

    impl Eq for RcSlice {}

    impl ::std::hash::Hash for RcSlice {
        fn hash<H>(&self, state: &mut H) where H: ::std::hash::Hasher {
            use std::mem::transmute;
            self.get_slice().hash(state)
        }
    }
}

