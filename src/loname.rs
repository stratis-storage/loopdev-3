//! Configuration structures for loop device

use std::path::Path;

use crate::bindings::LO_NAME_SIZE;

/// Loop device name
#[repr(C)]
#[derive(Debug)]
pub struct Name(pub [libc::__u8; LO_NAME_SIZE as usize]);

/// Allow to construct the config easily
impl Default for Name {
    fn default() -> Self {
        Self([0; LO_NAME_SIZE as usize])
    }
}

/// Conversions simplifiers
impl Name {
    pub fn from_path<Y: AsRef<Path>>(path: Y) -> Result<Self, String> {
        let s = path.as_ref().as_os_str().as_encoded_bytes();
        if s.len() > LO_NAME_SIZE as usize {
            return Err(format!(
                "too many bytes in the provided loop dev source file path: {}, max {LO_NAME_SIZE}",
                s.len()
            ));
        }
        let mut data: [u8; 64] = [0; LO_NAME_SIZE as usize];
        for (idx, byte) in s.into_iter().enumerate() {
            data[idx] = *byte;
        }
        Ok(Self(data))
    }
}
impl ToString for Name {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .filter_map(|ch| {
                let ch: char = *ch as char;
                if ch == '\0' {
                    None
                } else {
                    Some(ch)
                }
            })
            .collect::<String>()
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::Name;

    #[test]
    fn test_name_empty() {
        let name = Name::default();
        for num in name.0 {
            assert_eq!(0, num);
        }
    }

    #[test]
    fn test_name_from_to() {
        let path_string = "/a/b/some-file/cool name";
        let path = PathBuf::from(&path_string);
        let name = Name::from_path(path);

        assert_eq!(path_string, name.unwrap().to_string());
    }

    #[test]
    fn test_name_too_long() {
        let path_string = "/too-long/too-long/too-long/too-long/too-long/too-long/too-long--";
        let path = PathBuf::from(&path_string);
        let name = Name::from_path(path);

        assert_eq!(
            "too many bytes in the provided loop dev source file path: 65, max 64",
            name.unwrap_err()
        )
    }
}
