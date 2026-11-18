use crate::engine::{Context, Value};
use exe::{ImageDirectoryEntry, PE};
use phf::phf_map;

pub type Global = fn(&Context) -> Value;

pub const GLOBALS: phf::Map<&'static str, Global> = phf_map! {
    "ADDRESS_SPACE" => address_space,
    "RUNTIME_FUNCTION_TABLE" => runtime_function_table
};

fn address_space(ctx: &Context) -> Value {
    let base = ctx.file.get_image_base().unwrap();
    let sections = ctx.file.get_section_table().unwrap();

    Value::AddressRange(
        sections.iter()
            .map(|x| (
                base + x.virtual_address.0 as u64, 
                base + x.virtual_address.0 as u64 + x.size_of_raw_data as u64
            ))
            .filter(|(s, e)| s != e)
            .collect()
    )
}

fn runtime_function_table(ctx: &Context) -> Value {
    let base = ctx.file.get_image_base().unwrap();
    let dd = ctx.file.get_data_directory(ImageDirectoryEntry::Exception).unwrap();
    let section = ctx.file.get_section_by_rva(dd.virtual_address).unwrap();
    let data = section.read(ctx.file).unwrap();
    let data_offset = (dd.virtual_address.0 - section.virtual_address.0) as usize;

    let count = (dd.size / 12) as usize;
    let mut funcs = Vec::with_capacity(count as usize);
    for i in 0..count {
        let offset = data_offset + (i*12);
        let start_rva = u32::from_le_bytes(data[offset..(offset+4)].try_into().unwrap());
        let end_rva = u32::from_le_bytes(data[(offset+4)..(offset+8)].try_into().unwrap());
        funcs.push(Value::AddressRange([(base + start_rva as u64, base + end_rva as u64)].into()));
    }

    Value::List(funcs.into())
}
