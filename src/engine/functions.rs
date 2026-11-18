use crate::engine::{Context, Value};
use exe::{Buffer, PE, VA, VA64};
use phf::phf_map;

pub type Function = fn(&Context, &[Value]) -> Value;

pub const FUNCTIONS: phf::Map<&'static str, Function> = phf_map! {
    "bytes_match" => bytes_match,
    "modrm_references" => modrm_references
};

fn bytes_match(ctx: &Context, args: &[Value]) -> Value {
    let Value::Address(addr) = args[0] else { panic!("type error: addr") };
    let Value::Bytes(target) = &args[1] else { panic!("type error: target") };

    let offset = ctx.file.va_to_offset(VA::VA64(VA64(addr))).unwrap();
    let bytes = ctx.file.read(offset.0 as usize, target.len()).unwrap();

    Value::Bool(*bytes == **target)
}

fn modrm_references(ctx: &Context, args: &[Value]) -> Value {
    let sources = &args[0];
    let targets = &args[1];

    for s in sources.iter() {
        let Value::Address(s) = s else { panic!("type error: *sources"); };

        let offset = ctx.file.va_to_offset(VA::VA64(VA64(s))).unwrap();
        let bytes = ctx.file.read(offset.0 as usize, 4).unwrap();
        let potential_ref = i32::from_le_bytes(bytes.try_into().unwrap());
        
        for t in targets.iter() {
            let Value::Address(t) = t else { panic!("type error: *targets") };
            let wouldbe_ref = (t as i64 - (s + 4) as i64) as i32;

            if potential_ref == wouldbe_ref {
                return Value::Bool(true);
            }
        }
    }

    Value::Bool(false)
}
