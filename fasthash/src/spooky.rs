use std::hash::Hasher;
use std::os::raw::c_void;

use extprim::u128::u128;

use ffi;

use hasher::{FastHash, HasherExt};

#[doc(hidden)]
pub struct SpookyHash32 {}

impl FastHash for SpookyHash32 {
    type Value = u32;
    type Seed = u32;

    #[inline]
    fn hash_with_seed<T: AsRef<[u8]>>(bytes: &T, seed: u32) -> u32 {
        let mut hash1 = seed as u64;
        let mut hash2 = seed as u64;

        unsafe {
            ffi::SpookyHasherHash(bytes.as_ref().as_ptr() as *const c_void,
                                  bytes.as_ref().len(),
                                  &mut hash1,
                                  &mut hash2);
        }

        hash1 as u32
    }
}

#[doc(hidden)]
pub struct SpookyHash64 {}

impl FastHash for SpookyHash64 {
    type Value = u64;
    type Seed = u64;

    #[inline]
    fn hash_with_seed<T: AsRef<[u8]>>(bytes: &T, seed: u64) -> u64 {
        let mut hash1 = seed;
        let mut hash2 = seed;

        unsafe {
            ffi::SpookyHasherHash(bytes.as_ref().as_ptr() as *const c_void,
                                  bytes.as_ref().len(),
                                  &mut hash1,
                                  &mut hash2);
        }

        hash1
    }
}

#[doc(hidden)]
pub struct SpookyHash128 {}

impl FastHash for SpookyHash128 {
    type Value = u128;
    type Seed = u128;

    #[inline]
    fn hash_with_seed<T: AsRef<[u8]>>(bytes: &T, seed: u128) -> u128 {
        let mut hash1 = seed.high64();
        let mut hash2 = seed.low64();

        unsafe {
            ffi::SpookyHasherHash(bytes.as_ref().as_ptr() as *const c_void,
                                  bytes.as_ref().len(),
                                  &mut hash1,
                                  &mut hash2);
        }

        u128::from_parts(hash1, hash2)
    }
}

pub struct SpookyHasher64(*mut c_void);

impl SpookyHasher64 {
    #[inline]
    pub fn new() -> SpookyHasher64 {
        Self::with_seed(0)
    }

    #[inline]
    pub fn with_seed(seed: u64) -> SpookyHasher64 {
        let h = unsafe { ffi::SpookyHasherNew() };

        unsafe {
            ffi::SpookyHasherInit(h, seed, seed);
        }

        SpookyHasher64(h)
    }
}

impl Default for SpookyHasher64 {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher for SpookyHasher64 {
    #[inline]
    fn finish(&self) -> u64 {
        let mut hash1 = 0_u64;
        let mut hash2 = 0_u64;

        unsafe {
            ffi::SpookyHasherFinal(self.0, &mut hash1, &mut hash2);
        }

        hash1
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        unsafe {
            ffi::SpookyHasherUpdate(self.0,
                                    bytes.as_ref().as_ptr() as *const c_void,
                                    bytes.as_ref().len())
        }
    }
}

pub struct SpookyHasher128(*mut c_void);

impl SpookyHasher128 {
    #[inline]
    pub fn new() -> SpookyHasher128 {
        Self::with_seed(u128::new(0))
    }

    #[inline]
    pub fn with_seed(seed: u128) -> SpookyHasher128 {
        let h = unsafe { ffi::SpookyHasherNew() };

        unsafe {
            ffi::SpookyHasherInit(h, seed.high64(), seed.low64());
        }

        SpookyHasher128(h)
    }
}

impl Default for SpookyHasher128 {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SpookyHasher128 {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::SpookyHasherFree(self.0) }
    }
}

impl Hasher for SpookyHasher128 {
    #[inline]
    fn finish(&self) -> u64 {
        self.finish_ext().low64()
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        unsafe {
            ffi::SpookyHasherUpdate(self.0,
                                    bytes.as_ref().as_ptr() as *const c_void,
                                    bytes.as_ref().len())
        }
    }
}

impl HasherExt for SpookyHasher128 {
    #[inline]
    fn finish_ext(&self) -> u128 {
        let mut hash1 = 0_u64;
        let mut hash2 = 0_u64;

        unsafe {
            ffi::SpookyHasherFinal(self.0, &mut hash1, &mut hash2);
        }

        u128::from_parts(hash1, hash2)
    }
}

#[inline]
pub fn hash32<T: AsRef<[u8]>>(v: &T) -> u32 {
    SpookyHash32::hash(v)
}

#[inline]
pub fn hash32_with_seed<T: AsRef<[u8]>>(v: &T, seed: u32) -> u32 {
    SpookyHash32::hash_with_seed(v, seed)
}

#[inline]
pub fn hash64<T: AsRef<[u8]>>(v: &T) -> u64 {
    SpookyHash64::hash(v)
}

#[inline]
pub fn hash64_with_seed<T: AsRef<[u8]>>(v: &T, seed: u64) -> u64 {
    SpookyHash64::hash_with_seed(v, seed)
}

#[inline]
pub fn hash128<T: AsRef<[u8]>>(v: &T) -> u128 {
    SpookyHash128::hash(v)
}

#[inline]
pub fn hash128_with_seed<T: AsRef<[u8]>>(v: &T, seed: u128) -> u128 {
    SpookyHash128::hash_with_seed(v, seed)
}

#[cfg(test)]
mod tests {
    use std::hash::Hasher;

    use extprim::u128::u128;

    use hasher::{FastHash, HasherExt};
    use super::*;

    #[test]
    fn test_spooky32() {
        assert_eq!(SpookyHash32::hash(b"hello"), 3907268544);
        assert_eq!(SpookyHash32::hash_with_seed(b"hello", 123), 2211835972);
        assert_eq!(SpookyHash32::hash(b"helloworld"), 3874077464);
    }

    #[test]
    fn test_spooky64() {
        assert_eq!(SpookyHash64::hash(b"hello"), 6105954949053820864);
        assert_eq!(SpookyHash64::hash_with_seed(b"hello", 123),
                   8819086853393477700);
        assert_eq!(SpookyHash64::hash(b"helloworld"), 18412934266828208920);

        let mut h = SpookyHasher64::new();

        h.write(b"hello");
        assert_eq!(h.finish(), 6105954949053820864);

        h.write(b"world");
        assert_eq!(h.finish(), 18412934266828208920);
    }

    #[test]
    fn test_spooky128() {
        assert_eq!(SpookyHash128::hash(b"hello"),
                   u128::from_parts(6105954949053820864, 16417113279381893933));
        assert_eq!(SpookyHash128::hash_with_seed(b"hello", u128::new(123)),
                   u128::from_parts(7262466432451564128, 15030932129358977799));
        assert_eq!(SpookyHash128::hash(b"helloworld"),
                   u128::from_parts(18412934266828208920, 13883738476858207693));

        let mut h = SpookyHasher128::new();

        h.write(b"hello");
        assert_eq!(h.finish_ext(),
                   u128::from_parts(6105954949053820864, 16417113279381893933));

        h.write(b"world");
        assert_eq!(h.finish_ext(),
                   u128::from_parts(18412934266828208920, 13883738476858207693));
    }
}
