#![cfg(test)]
use std::{fs::File, path::Path, path::PathBuf};
use MediaStreamTree;

#[test]
fn visual_object_sequence() {
    use ::objects::ObjectBuilder;
    let mut handle = handle("01_simple.mp4");
    let stsd = handle.searchtree_stype::<::iso_p12::Stsd>("moov.trak2.mdia.minf.stbl.stsd").unwrap();
    for sample in stsd.sample_entries {
        match sample.downcast_ref::<::sample_entries::MP4VisualSampleEntry>() {
            Some(val) => {
                let val: &::sample_entries::MP4VisualSampleEntry = val;
                for descr in &val.esds_box.as_ref().unwrap().es.as_ref().unwrap().descriptors {
                    let decconfig: &::base_descriptors::DecoderConfigDescriptor = descr.downcast_ref::<::base_descriptors::DecoderConfigDescriptor>().unwrap();
                    for descr in &decconfig.descriptors {
                        let decspec: &::base_descriptors::DecoderSpecificInfo = descr.downcast_ref::<::base_descriptors::DecoderSpecificInfo>().unwrap();
                        ::objects::VisualObjectSequence::build(&decspec.extension);
                    }
                }
            }
            None => {}
        }
    }
}

fn path(filename: &str) -> PathBuf {
    /*D:\download.tsi.telecom-paristech.fr\gpac\MPEG\ISOBMFF-Conformance\isobmff*/
    let path = Path::new("d:\\")
        .join("download.tsi.telecom-paristech.fr")
        .join("gpac")
        .join("MPEG")
        .join("ISOBMFF-Conformance")
        .join("isobmff")
        .join(filename);
    path.to_path_buf()
}

fn handle(filename: &str) -> File {
    File::open(path(filename)).expect("Error opening file")
}