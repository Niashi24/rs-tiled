use alloc::string::ToString;
use alloc::vec::Vec;
use core::str::FromStr;

use crate::error::Error;

/// The Wang ID, stored as an array of 8 u8 values.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct WangId(pub [u8; 8]);

impl FromStr for WangId {
    type Err = Error;

    fn from_str(s: &str) -> core::result::Result<WangId, Error> {
        let mut ret = [0u8; 8];
        let values: Vec<&str> = s
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .collect();
        if values.len() != 8 {
            return Err(Error::InvalidWangIdEncoding {
                read_string: s.to_string(),
            });
        }
        for i in 0..8 {
            ret[i] = values[i].parse::<u8>().unwrap_or(0);
        }

        Ok(WangId(ret))
    }
}

/// Stores the Wang ID.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct WangTile {
    #[allow(missing_docs)]
    pub wang_id: WangId,
}
