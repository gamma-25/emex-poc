use std::rc::Rc;

use exe::VecPE;

pub mod functions;
pub mod globals;
pub mod pipeline;

pub use pipeline::Pipeline;

#[derive(Debug)]
pub struct Context<'a> {
    pub file: &'a VecPE
}

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Address(u64),
    Bytes(Rc<[u8]>),

    AddressRange(Rc<[(u64, u64)]>),
    List(Rc<[Value]>)
}

impl Value {
    pub fn iter(&self) -> ValueIterator<'_> {
        ValueIterator(self, 0)
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(b) 
                => write!(f, "{b}"),
            Value::Address(a) 
                => write!(f, "0x{a:x}"),
            Value::Bytes(b) 
                => write!(f, "{}", String::from_utf8_lossy(&b)),
            Value::AddressRange(r) => {
                write!(f, "Ranges[")?;
                for (s, e) in r.iter() {
                    write!(f, "0x{s:x}-0x{e:x},")?;
                }
                write!(f, "]")
            },
            Value::List(l) => {
                write!(f, "List[")?;
                for v in l.iter() {
                    write!(f, "{v},")?;
                }
                write!(f, "]")
            },
        }
    }
}

pub struct ValueIterator<'a>(&'a Value, usize);

impl<'a> Iterator for ValueIterator<'a> {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.0 {
            Value::Bool(_)    |
            Value::Address(_) |
            Value::Bytes(_)  => if self.1 == 0 { Some(self.0.clone()) } else { None },

            Value::AddressRange(spans) => {
                let mut offset = self.1;
                let mut start = None;
                for span in spans.iter() {
                    let size = (span.1 - span.0) as usize;
                    if offset < size {
                        start = Some(span.0);
                        break;
                    }
                    offset -= size as usize;
                }
                start.map(|x| Value::Address(x + offset as u64))
            },
            Value::List(items) => items.get(self.1).cloned()
        };

        if ret.is_some() {
            self.1 += 1;
        }
        ret
    }
}
