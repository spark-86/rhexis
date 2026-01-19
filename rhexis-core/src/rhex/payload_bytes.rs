// rhex_payload_bytes.rs
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_bytes::{ByteBuf, Bytes};

pub fn serialize<S>(data: &Vec<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Serialize as array of CBOR byte strings (not array-of-ints)
    let bytes: Vec<&Bytes> = data.iter().map(|v| Bytes::new(v)).collect();
    bytes.serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserialize array of byte strings into Vec<Vec<u8>>
    let bufs: Vec<ByteBuf> = Vec::<ByteBuf>::deserialize(deserializer)?;
    Ok(bufs.into_iter().map(|b| b.into_vec()).collect())
}
