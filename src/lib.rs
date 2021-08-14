//! Superfast id generator
//!
//! # Example
//!
//! ```
//! use hyperid::HyperId;
//!
//! let mut hyperid = HyperId::default();
//!
//! let id1 = hyperid.generate();
//! let id2 = hyperid.generate();
//!
//! assert_ne!(id1, id2);
//! #[cfg(feature = "url_safe")]
//! println!("{}", id1.to_url_safe()); // prints "100300792492935192884946730361868995562-15"
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]
// Import uuid
use uuid::Uuid;

/// Id generator. Every instance create different generator.
/// ```
/// use hyperid::HyperId;
/// let mut hyperid = HyperId::default();
///
/// let id = hyperid.generate();
/// let id2 = hyperid.generate();
///
/// assert_ne!(id, id2);
///
/// let id = hyperid.get();
/// let id2 = hyperid.get();
///
/// assert_eq!(id, id2);
/// ```
pub struct HyperId {
    uuid: Uuid,
    c: u8,
}

impl HyperId {
    /// Create a new HyperId instance
    /// ```
    /// use hyperid::HyperId;
    /// let mut hyperid = HyperId::new();
    /// ```
    pub fn new() -> Self {
        let uuid = Uuid::new_v4();
        let c: u8 = 0;

        Self { uuid, c }
    }

    /// Create a new HyperId instance starting from a given id
    /// ```
    /// use hyperid::{HyperId, Id};
    /// let bytes = vec![0; 17];
    /// let id = Id::from_bytes(bytes).unwrap();
    /// let hyperid = HyperId::from_id(id);
    /// let id = hyperid.get();
    /// assert_eq!(vec![0; 17], id.into_bytes());
    /// ```
    pub fn from_id(id: Id) -> Self {
        let uuid = id.uuid_as_128;
        let uuid = Uuid::from_u128(uuid);
        let c = id.c;

        Self { uuid, c }
    }

    /// Return the latest generated Id
    /// ```
    /// use hyperid::HyperId;
    /// let mut hyperid = HyperId::new();
    /// let id1 = hyperid.get();
    /// let id2 = hyperid.get();
    /// assert_eq!(id1, id2);
    /// ```
    pub fn get(&self) -> Id {
        Id {
            uuid_as_128: self.uuid.as_u128(),
            c: self.c,
        }
    }

    /// Generate the Id and returns it
    /// ```
    /// use hyperid::HyperId;
    /// let mut hyperid = HyperId::new();
    /// let id1 = hyperid.get();
    /// let id2 = hyperid.generate();
    /// assert_ne!(id1, id2);
    /// ```
    pub fn generate(&mut self) -> Id {
        self.c = self.c.checked_add(1).unwrap_or(0);
        if self.c == 0 {
            self.uuid = Uuid::new_v4();
        }

        Id {
            uuid_as_128: self.uuid.as_u128(),
            c: self.c,
        }
    }
}

impl Default for HyperId {
    fn default() -> Self {
        Self::new()
    }
}
/// Structure for keeping data
#[derive(PartialEq, Debug, Clone, Copy, Eq, Hash)]
pub struct Id {
    uuid_as_128: u128,
    c: u8,
}

impl Id {
    /// Return a bytes representation of id
    /// ```
    /// use hyperid::HyperId;
    /// let mut hyperid = HyperId::new();
    /// let id = hyperid.get();
    /// println!("{:?}", id.into_bytes());
    /// ```
    pub fn into_bytes(self) -> Vec<u8> {
        let uuid_as_128 = self.uuid_as_128;
        let mut bytes = uuid_as_128.to_be_bytes().to_vec();
        bytes.push(self.c);
        bytes
    }

    /// Build Id instance from bytes
    /// ```
    /// use hyperid::{HyperId, Id};
    /// let bytes = vec![0; 17];
    /// let id = Id::from_bytes(bytes).unwrap();
    /// assert_eq!(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], id.into_bytes());
    /// ```
    pub fn from_bytes(mut bytes: Vec<u8>) -> Result<Self, ParseIdError> {
        if bytes.len() != 17 {
            return Err(ParseIdError::WrongByteSize);
        }
        let c = bytes.pop().unwrap();
        let mut arr = [0u8; 16];
        bytes.swap_with_slice(&mut arr);
        Ok(Self {
            uuid_as_128: u128::from_be_bytes(arr),
            c,
        })
    }

    /// Return an url safe string
    /// ```
    /// use hyperid::HyperId;
    /// let mut hyperid = HyperId::new();
    /// let id = hyperid.get();
    /// println!("{}", id.to_url_safe()); // 3ZAYYJilG7vHTqiUuaQdFg.0
    /// ```
    #[cfg_attr(docsrs, doc(cfg(feature = "url_safe")))]
    #[cfg(feature = "url_safe")]
    pub fn to_url_safe(&self) -> String {
        let uuid_as_bytes = self.uuid_as_128.to_le_bytes();
        let str = base64::encode_config(uuid_as_bytes, base64::URL_SAFE_NO_PAD);
        format!("{}.{}", str, self.c)
    }

    /// Return an url safe string
    /// ```
    /// use hyperid::{HyperId, Id};
    /// let mut hyperid = HyperId::new();
    /// let id1 = hyperid.get();
    /// let s = id1.to_url_safe();
    /// let id2 = Id::from_url_safe(s).unwrap();
    /// assert_eq!(id1, id2);
    /// ```
    #[cfg_attr(docsrs, doc(cfg(feature = "url_safe")))]
    #[cfg(feature = "url_safe")]
    pub fn from_url_safe(s: String) -> Result<Id, ParseIdError> {
        let mut split = s.split('.');
        let uuid_as_128 = split
            .next()
            .ok_or(ParseIdError::NoBaseFound)
            .and_then(|uuid_as_128| {
                base64::decode_config(uuid_as_128, base64::URL_SAFE_NO_PAD)
                    .or(Err(ParseIdError::NoBaseFound))
                    .map(|mut v| {
                        let mut arr = [0u8; 16];
                        v.swap_with_slice(&mut arr);
                        arr
                    })
                    .map(u128::from_le_bytes)
            });
        let c = split
            .next()
            .ok_or(ParseIdError::NoCounterFound)
            .and_then(|c| c.parse::<u8>().map_err(|_| ParseIdError::NoCounterFound));

        match (uuid_as_128, c) {
            (Ok(uuid_as_128), Ok(c)) => Ok(Id { uuid_as_128, c }),
            (Err(err), _) | (_, Err(err)) => Err(err),
        }
    }
}

#[derive(Debug)]
pub enum ParseIdError {
    NoBaseFound,
    NoCounterFound,
    WrongByteSize,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn starts_from_zero() {
        let hyperid = HyperId::default();
        assert_eq!(hyperid.c, 0);
    }

    #[test]
    fn get_return_equal_id() {
        let hyperid = HyperId::default();
        assert_eq!(hyperid.get(), hyperid.get());
    }

    #[test]
    fn generate_change_internal_state() {
        let mut hyperid = HyperId::default();

        let c = hyperid.c;
        hyperid.generate();
        assert_ne!(hyperid.c, c);
    }

    #[test]
    fn generate_returns_different_id() {
        let mut hyperid = HyperId::default();

        let previous_id = hyperid.get();
        let next_id = hyperid.generate();
        assert_ne!(previous_id, next_id);
    }

    #[test]
    fn on_255_generate_a_new_base() {
        let mut hyperid = HyperId::default();

        let base = hyperid.uuid;

        for _ in 0..255 {
            hyperid.generate();
        }
        let new_base = hyperid.uuid;
        assert_eq!(base, new_base);

        hyperid.generate();

        let new_base = hyperid.uuid;
        assert_ne!(base, new_base);
    }

    #[test]
    fn different_instances_have_different_base() {
        let hyperid1 = HyperId::default();
        let hyperid2 = HyperId::default();
        assert_ne!(hyperid1.uuid, hyperid2.uuid);
    }

    #[test]
    fn into_bytes() {
        let hyperid = HyperId::default();

        let id = hyperid.get();

        let id_bytes = id.into_bytes();

        let id_from_decode = Id::from_bytes(id_bytes).unwrap();

        assert_eq!(hyperid.get(), id_from_decode);
    }

    
    #[test]
    fn id_could_be_the_key_of_hashmap() {
        let hyperid = HyperId::default();

        let id = hyperid.get();

        let mut map = HashMap::new();
        map.insert(id, true);
    }

    #[cfg(feature = "url_safe")]
    #[test]
    fn url_safe_encode_decode() {
        let hyperid = HyperId::default();

        let id = hyperid.get();

        let id_str = id.to_url_safe();

        let id_from_decode = Id::from_url_safe(id_str).unwrap();

        assert_eq!(id, id_from_decode);
    }
}
