use {BuildNode, IsSlice, Name};

#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct Stsd {
    entry_count: Option<u32>,
}

impl<'a> Name<'a> for Stsd {
    fn name() -> &'a str {
        "stsd"
    }
}

impl<'a> BuildNode for Stsd {
    fn build<T: IsSlice<Item = u8>>(data: T) -> Option<Self> {
        let data = data.as_slice();
        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::Cursor;
        let entry_count = Cursor::new(&data[12..16]).read_u32::<BigEndian>().ok();
        Some(Stsd {
            entry_count,
            ..Default::default()
        })
    }
}
