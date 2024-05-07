use uuid::Uuid;

const BASE_UUID: (u32, u16, u16, &[u8; 8]) = (0, 0, 0x1000, b"\x80\x00\x00\x80\x5F\x9B\x34\xFB");

pub trait SdpShortUuid<T: Into<u32>> {
    fn from_sdp_short_uuid(uuid: T) -> Uuid {
        uuid_from_sdp(uuid.into())
    }
}

pub const fn uuid_from_sdp(uuid: u32) -> Uuid{
    Uuid::from_fields(uuid, BASE_UUID.1, BASE_UUID.2, BASE_UUID.3)
}

impl SdpShortUuid<u16> for Uuid {}
impl SdpShortUuid<u32> for Uuid {}
