use super::{descrfactory, size_of_instance, DescrBase, DescrBaseTags, DescrBuilder};
use IsSlice;
#[repr(align(8))]
#[derive(Debug, Default, Clone)]
pub struct DecoderConfigDescriptor {
    tag: Option<DescrBaseTags>,
    size_of_instance: Option<u8>,
    objecttypeindication: Option<u8>,
    streamtype: Option<[bool; 6]>,
    upstream: Option<bool>,
    reserved: Option<bool>,
    buffersize_db: Option<[bool; 24]>,
    max_bit_rate: Option<u32>,
    avg_bit_rate: Option<u32>,
    descriptors: Vec<Box<dyn DescrBase>>,
}

impl DescrBase for DecoderConfigDescriptor {
    fn tag(&self) -> Option<DescrBaseTags> {
        self.tag.clone()
    }

    fn rdclone(&self) -> Box<DescrBase> {
        Box::new(self.clone())
    }
}

impl DescrBuilder for DecoderConfigDescriptor {
    fn build<T: IsSlice<Item = u8>>(d: T) -> Option<Self> {
        let data = d.as_slice();
        use byteorder::{BigEndian, ReadBytesExt};
        use std::io::Cursor;
        let tag = Some(match Cursor::new(&data[..1])
            .read_u8()
            .expect("DecoderConfigDescriptor error reading tag")
        {
            0x04 => DescrBaseTags::DecoderConfigDescrTag,
            val => {
                panic!(format!("DecoderConfigDescriptor tag doesn't match the object descriptor base tags, {:?}", val));
            }
        });

        let mut cursor: usize = 1;
        let size_of_instance = Some(size_of_instance(data, &mut cursor));
        let objecttypeindication = Cursor::new(&data[cursor..cursor+1]).read_u8().ok();
        let byte = Cursor::new(&data[cursor..cursor+1]).read_u8().expect("DecoderConfigDescriptor error reading bytes");
        let streamtype = Some({
            let mut arr_idx = 0;
            let mut ret = [false; 6];
            for idx in (2..8).rev() {
                ret[arr_idx] = byte & (1 << idx) > 0;
                arr_idx+=1;
            }
            ret
        });
        let upstream = Some(byte & (1 << 1) > 0);
        let reserved = Some(byte & (1 << 0) > 0);
        let byte = Cursor::new(&data[cursor..cursor+4]).read_u32::<BigEndian>().expect("DecoderConfigDescriptor error reading bytes");
        let buffersize_db = Some({
            let mut arr_idx = 0;
            let mut ret = [false; 24];
            for idx in (8..24).rev() {
                ret[arr_idx] = byte & (1 << idx) > 0;
                arr_idx+=1;
            }
            ret
        });
        let max_bit_rate = Cursor::new(&data[cursor+4..cursor+8]).read_u32::<BigEndian>().ok();
        let avg_bit_rate = Cursor::new(&data[cursor+8..cursor+12]).read_u32::<BigEndian>().ok();
        println!("Building descriptors!");
        let descriptors = descrfactory(&data[12..]);
        Some(DecoderConfigDescriptor {
            tag,
            size_of_instance,
            objecttypeindication,
            streamtype,
            upstream,
            reserved,
            buffersize_db,
            max_bit_rate,
            avg_bit_rate,
            descriptors,
            ..Default::default()
        })
    }
}