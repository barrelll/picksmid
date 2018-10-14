//object descriptor's defined in iso 14496 part 1
mod object_descriptor_update;
mod object_descriptor_remove;

enum DescriptorClassTag {
    Forbidden1 = 0x00,
    ObjectDescrTag = 0x01,
    InitialObjectDescrTag = 0x02,
    ESDescrTag = 0x03,
    DecoderConfigDescrTag = 0x04,
    DecSpecificInfoTag = 0x05,
    SLConfigDescrTag = 0x06,
    ContentIdentDescrTag = 0x07,
    SupplContentIdentDescrTag = 0x08,
    IPIDescrPointerTag = 0x09,
    IPMPDescrPointerTag = 0x0A,
    IPMPDescrTag = 0x0B,
    QoSescrTag = 0x0C,
    RegistrationDescrTag = 0x0D,
    ESIDIncTag = 0x0E,
    ESIDRefTag = 0x0F,
    MP4IODTag = 0x10,
    MP4ODTag = 0x11,
    IPLDescrPointerRefTag = 0x12,
    ExtensionProfileLevelDescrTag = 0x13,
    ProfileLevelIndicationIndexDescrTag = 0x14,
    Reserved1 = 0x15 - 0x3F,
    ContentClassificationDescrTag = 0x40,
    KeyWordDescrTag = 0x41,
    RatingDescrTag = 0x42,
    LanguageDescrTag = 0x43,
    ShortTextualDescrTag = 0x44,
    ExpandedTextualDescrTag = 0x45,
    ContentCreatorNameDescrTag = 0x46,
    ContentCreationDateDescrTag = 0x47,
    OCICreatorNameDescrTag = 0x48,
    OCICreationDateDescrTag = 0x49,
    SmpteCameraPositionDescrTag = 0x4A,
    SegmentDescrTag = 0x4B,
    MediaTimeDescrTag = 0x4C,
    Reserved2 = 0x4D - 0x5F,
    IPMPToolsListDescrTag = 0x60,
    IPMPToolTag = 0x61,
    M4MuxTimingDescrTag = 0x62,
    M4MuxCodeTableDescrTag = 0x63,
    ExtSLConfigDescrTag = 0x64,
    M4MuxBufferSizeDescrTag = 0x65,
    M4MuxIdentDescrTag = 0x66,
    DependencyPointerTag = 0x67,
    DependencyMarkerTag = 0x68,
    M4MuxChannelDescrTag = 0x69,
    Reserved3 = 0x6A - 0xBF,
    UserPrivate = 0xC0 - 0xFE,
    Forbidden2 = 0xFF,
}

enum DescriptorCommandTag {
    Forbidden1 = 0x00,
    ObjectDescrUpdateTag = 0x01,
    ObjectDescrRemoveTag = 0x02,
    ESDescrUpdateTag = 0x03,
    ESDescrRemoveTag = 0x04,
    IPMPDescrUpdateTag = 0x05,
    IPMPDescrRemoveTag = 0x06,
    ESDescrRemoveRefTag = 0x07,
    ObjectDescrExecuteTag = 0x08,
    Reserved = 0x09-0xBF,
    UserPrivate = 0xC0-0xFE,
    Forbidden2 = 0xFF,
}

trait BaseDescriptor {
    fn tag(&self) -> DescriptorClassTag;
}

trait BaseCommand {
    fn tag(&self) -> DescriptorCommandTag;
}


trait ObjectDescriptorBase : BaseDescriptor {
    
}