use anyhow::Result;
use lz4_flex::{compress_prepend_size, decompress_size_prepended};
use serde::{Serialize, de::DeserializeOwned};

pub fn serialize(val: impl Serialize) -> Result<Vec<u8>> {
    Ok(compress(&serde_json::to_string(&val)?.into_bytes()))
}

pub fn deserialize<T: DeserializeOwned>(buff: &[u8]) -> Result<T> {
    let data = decompress(buff);
    let json_str = std::str::from_utf8(&data)?;
    Ok(serde_json::from_str(json_str)?)
}

pub fn compress(buf: &[u8]) -> Vec<u8> {
    compress_prepend_size(buf)
}

pub fn decompress(buf: &[u8]) -> Vec<u8> {
    decompress_size_prepended(buf).unwrap()
}

#[cfg(test)]
mod test {

    use anyhow::Result;
    use serde::{Deserialize, Serialize};

    use crate::serde::{deserialize, serialize};

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    struct User {
        age:    i32,
        height: f32,
        name:   String,
    }

    #[test]
    fn test_compress() -> Result<()> {
        let users = vec![
            User {
                age:    55,
                height: 1.9,
                name:   "Roma".to_owned(),
            };
            100
        ];

        let string = serde_json::to_string(&users)?;

        let ser = serialize(&users)?;

        assert_eq!(string.len(), 3801);
        assert_eq!(ser.len(), 69);

        let de: Vec<User> = deserialize(&ser)?;

        assert_eq!(users, de);

        Ok(())
    }
}
