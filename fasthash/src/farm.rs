use std::mem;

use extprim::u128::u128;

use ffi;

use hasher::{Fingerprint, FastHash};

#[doc(hidden)]
pub struct FarmHash32 {}

impl FastHash for FarmHash32 {
    type Value = u32;
    type Seed = u32;

    #[inline]
    fn hash<T: AsRef<[u8]>>(bytes: &T) -> u32 {
        unsafe { ffi::farmhash32(bytes.as_ref().as_ptr() as *const i8, bytes.as_ref().len()) }
    }

    #[inline]
    fn hash_with_seed<T: AsRef<[u8]>>(bytes: &T, seed: u32) -> u32 {
        unsafe {
            ffi::farmhash32_with_seed(bytes.as_ref().as_ptr() as *const i8,
                                      bytes.as_ref().len(),
                                      seed)
        }
    }
}

impl_hasher!(FarmHasher32, FarmHash32);

#[doc(hidden)]
pub struct FarmHash64 {}

impl FarmHash64 {
    #[inline]
    pub fn hash_with_seeds<T: AsRef<[u8]>>(bytes: &T, seed0: u64, seed1: u64) -> u64 {
        unsafe {
            ffi::farmhash64_with_seeds(bytes.as_ref().as_ptr() as *const i8,
                                       bytes.as_ref().len(),
                                       seed0,
                                       seed1)
        }
    }
}

impl FastHash for FarmHash64 {
    type Value = u64;
    type Seed = u64;

    #[inline]
    fn hash<T: AsRef<[u8]>>(bytes: &T) -> u64 {
        unsafe { ffi::farmhash64(bytes.as_ref().as_ptr() as *const i8, bytes.as_ref().len()) }
    }

    #[inline]
    fn hash_with_seed<T: AsRef<[u8]>>(bytes: &T, seed: u64) -> u64 {
        unsafe {
            ffi::farmhash64_with_seed(bytes.as_ref().as_ptr() as *const i8,
                                      bytes.as_ref().len(),
                                      seed)
        }
    }
}

impl_hasher!(FarmHasher64, FarmHash64);

#[doc(hidden)]
pub struct FarmHash128 {}

impl FastHash for FarmHash128 {
    type Value = u128;
    type Seed = u128;

    #[inline]
    fn hash<T: AsRef<[u8]>>(bytes: &T) -> u128 {
        unsafe {
            mem::transmute(ffi::farmhash128(bytes.as_ref().as_ptr() as *const i8,
                                            bytes.as_ref().len()))
        }
    }

    #[inline]
    fn hash_with_seed<T: AsRef<[u8]>>(bytes: &T, seed: u128) -> u128 {
        unsafe {
            mem::transmute(ffi::farmhash128_with_seed(bytes.as_ref().as_ptr() as *const i8,
                                                      bytes.as_ref().len(),
                                                      mem::transmute(seed)))
        }
    }
}

impl_hasher_ext!(FarmHasher128, FarmHash128);

#[inline]
pub fn hash32<T: AsRef<[u8]>>(v: &T) -> u32 {
    FarmHash32::hash(v)
}

#[inline]
pub fn hash32_with_seed<T: AsRef<[u8]>>(v: &T, seed: u32) -> u32 {
    FarmHash32::hash_with_seed(v, seed)
}

#[inline]
pub fn hash64<T: AsRef<[u8]>>(v: &T) -> u64 {
    FarmHash64::hash(v)
}

#[inline]
pub fn hash64_with_seed<T: AsRef<[u8]>>(v: &T, seed: u64) -> u64 {
    FarmHash64::hash_with_seed(v, seed)
}

pub fn hash64_with_seeds<T: AsRef<[u8]>>(v: &T, seed0: u64, seed1: u64) -> u64 {
    FarmHash64::hash_with_seeds(v, seed0, seed1)
}

#[inline]
pub fn hash128<T: AsRef<[u8]>>(v: &T) -> u128 {
    FarmHash128::hash(v)
}

#[inline]
pub fn hash128_with_seed<T: AsRef<[u8]>>(v: &T, seed: u128) -> u128 {
    FarmHash128::hash_with_seed(v, seed)
}

#[inline]
pub fn fingerprint32<T: AsRef<[u8]>>(v: &T) -> u32 {
    unsafe { ffi::farmhash_fingerprint32(v.as_ref().as_ptr() as *const i8, v.as_ref().len()) }
}

#[inline]
pub fn fingerprint64<T: AsRef<[u8]>>(v: &T) -> u64 {
    unsafe { ffi::farmhash_fingerprint64(v.as_ref().as_ptr() as *const i8, v.as_ref().len()) }
}

#[inline]
pub fn fingerprint128<T: AsRef<[u8]>>(v: &T) -> u128 {
    unsafe {
        mem::transmute(ffi::farmhash_fingerprint128(v.as_ref().as_ptr() as *const i8,
                                                    v.as_ref().len()))
    }
}

impl Fingerprint<u64> for u64 {
    #[inline]
    fn fingerprint(&self) -> u64 {
        unsafe { ffi::farmhash_fingerprint_uint64(*self) }
    }
}

impl Fingerprint<u64> for u128 {
    #[inline]
    fn fingerprint(&self) -> u64 {
        unsafe { ffi::farmhash_fingerprint_uint128(mem::transmute(*self)) }
    }
}

#[cfg(test)]
mod tests {
    use std::hash::Hasher;

    use extprim::u128::u128;

    use hasher::{Fingerprint, FastHash, HasherExt};
    use super::*;

    #[test]
    fn test_farmhash32() {
        assert_eq!(FarmHash32::hash(b"hello"), 3111026382);
        assert_eq!(FarmHash32::hash_with_seed(b"hello", 123), 1449662659);
        assert_eq!(FarmHash32::hash(b"helloworld"), 3283552592);

        let mut h = FarmHasher32::new();

        h.write(b"hello");
        assert_eq!(h.finish(), 3111026382);

        h.write(b"world");
        assert_eq!(h.finish(), 3283552592);
    }

    #[test]
    fn test_farmhash64() {
        assert_eq!(FarmHash64::hash(b"hello"), 14403600180753024522);
        assert_eq!(FarmHash64::hash_with_seed(b"hello", 123),
                   6856739100025169098);
        assert_eq!(FarmHash64::hash_with_seeds(b"hello", 123, 456),
                   15077713332534145879);
        assert_eq!(FarmHash64::hash(b"helloworld"), 1077737941828767314);

        let mut h = FarmHasher64::new();

        h.write(b"hello");
        assert_eq!(h.finish(), 14403600180753024522);

        h.write(b"world");
        assert_eq!(h.finish(), 1077737941828767314);
    }

    #[test]
    fn test_farmhash128() {
        assert_eq!(FarmHash128::hash(b"hello"),
                   u128::from_parts(14545675544334878584, 15888401098353921598));
        assert_eq!(FarmHash128::hash_with_seed(b"hello", u128::new(123)),
                   u128::from_parts(15212901187400903054, 13320390559359511083));
        assert_eq!(FarmHash128::hash(b"helloworld"),
                   u128::from_parts(16066658700231169910, 1119455499735156801));

        let mut h = FarmHasher128::new();

        h.write(b"hello");
        assert_eq!(h.finish_ext(),
                   u128::from_parts(14545675544334878584, 15888401098353921598));

        h.write(b"world");
        assert_eq!(h.finish_ext(),
                   u128::from_parts(16066658700231169910, 1119455499735156801));
    }

    #[test]
    fn test_fingerprint() {
        assert_eq!(fingerprint32(b"hello word"), 4146030890);
        assert_eq!(fingerprint64(b"hello word"), 2862784602449412590_u64);
        assert_eq!(fingerprint128(b"hello word"),
                   u128::from_parts(3993975538242800734, 12454188156902618296));
        assert_eq!(123_u64.fingerprint(), 4781265650859502840);
        assert_eq!(u128::new(123).fingerprint(), 4011577241381678309);
    }
}
