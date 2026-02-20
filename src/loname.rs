//! Configuration structures for loop device

use std::{
    ffi::CStr,
    path::{Path, PathBuf},
};

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
    pub fn from_path<Y: AsRef<Path>>(path: &Y) -> Result<Self, String> {
        let s = path.as_ref().as_os_str().as_encoded_bytes();
        if s.len() > LO_NAME_SIZE as usize {
            return Err(format!(
                "too many bytes in the provided loop dev source file path: {}, max {LO_NAME_SIZE}",
                s.len()
            ));
        }
        let mut data: [u8; 64] = [0; LO_NAME_SIZE as usize];
        for (idx, byte) in s.iter().enumerate() {
            data[idx] = *byte;
        }
        Ok(Self(data))
    }
}

impl TryInto<PathBuf> for Name {
    type Error = NameToPathBufError;
    fn try_into(self) -> Result<PathBuf, Self::Error> {
        // cut trailing \0
        let c_str = CStr::from_bytes_until_nul(&self.0)
            .map_err(|_e| Self::Error::FromBytesUntilNulError)?;
        // UTF-8 decode
        let string = c_str.to_str().map_err(|_e| Self::Error::CStrToStr)?;
        if string.is_empty() {
            Err(Self::Error::EmptyName)
        } else {
            Ok(PathBuf::from(string))
        }
    }
}

#[derive(Debug)]
pub enum NameToPathBufError {
    EmptyName,
    FromBytesUntilNulError,
    CStrToStr,
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_name_empty() {
        let name = Name::default();
        for num in name.0 {
            assert_eq!(0, num);
        }
    }

    #[test]
    fn test_name_from_to() {
        let path = PathBuf::from("/a/b/some-file/かっこいい имя");
        let name = Name::from_path(&path);

        assert_eq!(
            path,
            <Name as TryInto<PathBuf>>::try_into(name.unwrap()).expect("path should not be empty")
        );
    }

    #[test]
    fn test_name_too_long() {
        let path_string = "/too-long/too-long/too-long/too-long/too-long/too-long/too-long--";
        let path = PathBuf::from(&path_string);
        let name = Name::from_path(&path);

        assert_eq!(
            "too many bytes in the provided loop dev source file path: 65, max 64",
            name.unwrap_err()
        )
    }

    #[test]
    fn test_name_is_empty() {
        let name = Name::default();

        assert!(matches!(
            <Name as TryInto<PathBuf>>::try_into(name),
            Err(NameToPathBufError::EmptyName)
        ));
    }

    #[test]
    fn test_name_non_utf8() {
        let name = Name([
            'H' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8, 159, 146, 150, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]);

        assert!(matches!(
            <Name as TryInto<PathBuf>>::try_into(name),
            Err(NameToPathBufError::CStrToStr)
        ));
    }
}
