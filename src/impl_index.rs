#[cfg(alloc)]
use alloc::string::String;
#[cfg(alloc)]
use alloc::vec::Vec;
use core::ops::Index;
use core::ops::IndexMut;
#[cfg(feature = "doc_item")]
use doc_item::docbox;
#[cfg(feature = "doc_item")]
use doc_item::since;
#[cfg(std)]
use std::ffi::CStr;
use RangeFromExclusive;
use RangeFromExclusiveToExclusive;
use RangeFromExclusiveToInclusive;


#[cfg_attr(feature = "doc_item", doc_item::since(content="1.41.0"))]
impl<T> Index<RangeFromExclusive<usize>> for [T] {
    type Output = [T];

    #[inline]
    fn index(&self, index: RangeFromExclusive<usize>) -> &[T] {
        if index.start == core::usize::MAX {
            panic!("attempted to index slice exclusively from maximum usize");
        }
        &self[(index.start + 1)..self.len()]
    }
}


#[cfg_attr(feature = "doc_item", since(content="1.41.0"))]
impl<T> IndexMut<RangeFromExclusive<usize>> for [T] {
    #[inline]
    fn index_mut(&mut self, index: RangeFromExclusive<usize>) -> &mut [T] {
        if index.start == core::usize::MAX {
            panic!("attempted to index slice exclusively from maximum usize");
        }
        let len = self.len();
        &mut self[(index.start + 1)..len]
    }
}


#[cfg_attr(feature = "doc_item", since(content="1.41.0"))]
impl Index<RangeFromExclusive<usize>> for str {
    type Output = str;

    #[inline]
    fn index(&self, index: RangeFromExclusive<usize>) -> &str {
        if index.start == core::usize::MAX {
            panic!("attempted to index slice exclusively from maximum usize");
        }
        &self[(index.start + 1)..self.len()]
    }
}


#[cfg_attr(feature = "doc_item", since(content="1.41.0"))]
impl IndexMut<RangeFromExclusive<usize>> for str {
    #[inline]
    fn index_mut(&mut self, index: RangeFromExclusive<usize>) -> &mut str {
        if index.start == core::usize::MAX {
            panic!("attempted to index slice exclusively from maximum usize");
        }
        let len = self.len();
        &mut self[(index.start + 1)..len]
    }
}

#[cfg(alloc)]
#[cfg_attr(feature = "doc_item", since(content="1.41.0"))]
impl Index<RangeFromExclusive<usize>> for String {
    type Output = str;

    #[inline]
    fn index(&self, index: RangeFromExclusive<usize>) -> &str {
        &self[..][index]
    }
}

#[cfg(alloc)]
#[cfg_attr(feature = "doc_item", since(content="1.41.0"))]
impl IndexMut<RangeFromExclusive<usize>> for String {
    #[inline]
    fn index_mut(&mut self, index: RangeFromExclusive<usize>) -> &mut str {
        &mut self[..][index]
    }
}

#[cfg(alloc)]
#[cfg_attr(feature = "doc_item", since(content="1.41.0"))]
impl<T> Index<RangeFromExclusive<usize>> for Vec<T> {
    type Output = [T];

    #[inline]
    fn index(&self, index: RangeFromExclusive<usize>) -> &[T] {
        Index::index(&**self, index)
    }
}

#[cfg(alloc)]
#[cfg_attr(feature = "doc_item", since(content="1.41.0"))]
impl<T> IndexMut<RangeFromExclusive<usize>> for Vec<T> {
    #[inline]
    fn index_mut(&mut self, index: RangeFromExclusive<usize>) -> &mut [T] {
        IndexMut::index_mut(&mut **self, index)
    }
}

#[cfg(std)]
#[cfg_attr(feature = "doc_item", since(content="1.41.0"))]
#[cfg_attr(
    feature = "doc_item",
    docbox(
        content = "Only available on targets supporting <b><code>std</code></b>.",
        class = "std"
    )
)]
impl Index<RangeFromExclusive<usize>> for CStr {
    type Output = CStr;

    #[inline]
    fn index(&self, index: RangeFromExclusive<usize>) -> &CStr {
        if index.start == core::usize::MAX {
            panic!("attempted to index slice exclusively from maximum usize");
        }
        let bytes = self.to_bytes_with_nul();
        // We need to manually check the starting index to account for the null byte, since
        // otherwise we could get an empty string that doesn't end in a null.
        if index.start + 1 < bytes.len() {
            unsafe {
                // SAFETY: We guaranteed above that these bytes will end with a null.
                CStr::from_bytes_with_nul_unchecked(&bytes[(index.start + 1)..])
            }
        } else {
            panic!(
                "index out of bounds: the len is {} but the index is {}",
                bytes.len(),
                index.start
            );
        }
    }
}


#[cfg_attr(feature = "doc_item", since(content="1.41.0"))]
impl<T> Index<RangeFromExclusiveToInclusive<usize>> for [T] {
    type Output = [T];

    #[inline]
    fn index(&self, index: RangeFromExclusiveToInclusive<usize>) -> &[T] {
        if index.start == core::usize::MAX {
            panic!("attempted to index slice exclusively from maximum usize");
        }
        if index.end == core::usize::MAX {
            panic!("attempted to index slice inclusively to maximum usize");
        }
        &self[(index.start + 1)..(index.end + 1)]
    }
}


#[cfg_attr(feature = "doc_item", since(content="1.41.0"))]
impl<T> IndexMut<RangeFromExclusiveToInclusive<usize>> for [T] {
    #[inline]
    fn index_mut(&mut self, index: RangeFromExclusiveToInclusive<usize>) -> &mut [T] {
        if index.start == core::usize::MAX {
            panic!("attempted to index slice exclusively from maximum usize");
        }
        if index.end == core::usize::MAX {
            panic!("attempted to index slice inclusively to maximum usize");
        }
        &mut self[(index.start + 1)..(index.end + 1)]
    }
}

#[cfg(alloc)]
#[cfg_attr(feature = "doc_item", since(content="1.41.0"))]
impl<T> Index<RangeFromExclusiveToInclusive<usize>> for Vec<T> {
    type Output = [T];

    #[inline]
    fn index(&self, index: RangeFromExclusiveToInclusive<usize>) -> &[T] {
        Index::index(&**self, index)
    }
}

#[cfg(alloc)]
#[cfg_attr(feature = "doc_item", since(content="1.41.0"))]
impl<T> IndexMut<RangeFromExclusiveToInclusive<usize>> for Vec<T> {
    #[inline]
    fn index_mut(&mut self, index: RangeFromExclusiveToInclusive<usize>) -> &mut [T] {
        IndexMut::index_mut(&mut **self, index)
    }
}


#[cfg_attr(feature = "doc_item", since(content="1.41.0"))]
impl Index<RangeFromExclusiveToInclusive<usize>> for str {
    type Output = str;

    #[inline]
    fn index(&self, index: RangeFromExclusiveToInclusive<usize>) -> &str {
        if index.start == core::usize::MAX {
            panic!("attempted to index slice exclusively from maximum usize");
        }
        if index.end == core::usize::MAX {
            panic!("attempted to index slice inclusively to maximum usize");
        }
        &self[(index.start + 1)..(index.end + 1)]
    }
}


#[cfg_attr(feature = "doc_item", since(content="1.41.0"))]
impl IndexMut<RangeFromExclusiveToInclusive<usize>> for str {
    #[inline]
    fn index_mut(&mut self, index: RangeFromExclusiveToInclusive<usize>) -> &mut str {
        if index.start == core::usize::MAX {
            panic!("attempted to index slice exclusively from maximum usize");
        }
        if index.end == core::usize::MAX {
            panic!("attempted to index slice inclusively to maximum usize");
        }
        &mut self[(index.start + 1)..(index.end + 1)]
    }
}

#[cfg(alloc)]
#[cfg_attr(feature = "doc_item", since(content="1.41.0"))]
impl Index<RangeFromExclusiveToInclusive<usize>> for String {
    type Output = str;

    #[inline]
    fn index(&self, index: RangeFromExclusiveToInclusive<usize>) -> &str {
        &self[..][index]
    }
}

#[cfg(alloc)]
#[cfg_attr(feature = "doc_item", since(content="1.41.0"))]
impl IndexMut<RangeFromExclusiveToInclusive<usize>> for String {
    #[inline]
    fn index_mut(&mut self, index: RangeFromExclusiveToInclusive<usize>) -> &mut str {
        &mut self[..][index]
    }
}


#[cfg_attr(feature = "doc_item", since(content="1.41.0"))]
impl<T> Index<RangeFromExclusiveToExclusive<usize>> for [T] {
    type Output = [T];

    #[inline]
    fn index(&self, index: RangeFromExclusiveToExclusive<usize>) -> &[T] {
        if index.start == core::usize::MAX {
            panic!("attempted to index slice exclusively from maximum usize");
        }
        &self[(index.start + 1)..index.end]
    }
}


#[cfg_attr(feature = "doc_item", since(content="1.41.0"))]
impl<T> IndexMut<RangeFromExclusiveToExclusive<usize>> for [T] {
    #[inline]
    fn index_mut(&mut self, index: RangeFromExclusiveToExclusive<usize>) -> &mut [T] {
        if index.start == core::usize::MAX {
            panic!("attempted to index slice exclusively from maximum usize");
        }
        &mut self[(index.start + 1)..index.end]
    }
}

#[cfg(alloc)]
#[cfg_attr(feature = "doc_item", since(content="1.41.0"))]
impl<T> Index<RangeFromExclusiveToExclusive<usize>> for Vec<T> {
    type Output = [T];

    #[inline]
    fn index(&self, index: RangeFromExclusiveToExclusive<usize>) -> &[T] {
        Index::index(&**self, index)
    }
}

#[cfg(alloc)]
#[cfg_attr(feature = "doc_item", since(content="1.41.0"))]
impl<T> IndexMut<RangeFromExclusiveToExclusive<usize>> for Vec<T> {
    #[inline]
    fn index_mut(&mut self, index: RangeFromExclusiveToExclusive<usize>) -> &mut [T] {
        IndexMut::index_mut(&mut **self, index)
    }
}


#[cfg_attr(feature = "doc_item", since(content="1.41.0"))]
impl Index<RangeFromExclusiveToExclusive<usize>> for str {
    type Output = str;

    #[inline]
    fn index(&self, index: RangeFromExclusiveToExclusive<usize>) -> &str {
        if index.start == core::usize::MAX {
            panic!("attempted to index slice exclusively from maximum usize");
        }
        &self[(index.start + 1)..index.end]
    }
}


#[cfg_attr(feature = "doc_item", since(content="1.41.0"))]
impl IndexMut<RangeFromExclusiveToExclusive<usize>> for str {
    #[inline]
    fn index_mut(&mut self, index: RangeFromExclusiveToExclusive<usize>) -> &mut str {
        if index.start == core::usize::MAX {
            panic!("attempted to index slice exclusively from maximum usize");
        }
        &mut self[(index.start + 1)..index.end]
    }
}

#[cfg(alloc)]
#[cfg_attr(feature = "doc_item", since(content="1.41.0"))]
impl Index<RangeFromExclusiveToExclusive<usize>> for String {
    type Output = str;

    #[inline]
    fn index(&self, index: RangeFromExclusiveToExclusive<usize>) -> &str {
        &self[..][index]
    }
}

#[cfg(alloc)]
#[cfg_attr(feature = "doc_item", since(content="1.41.0"))]
impl IndexMut<RangeFromExclusiveToExclusive<usize>> for String {
    #[inline]
    fn index_mut(&mut self, index: RangeFromExclusiveToExclusive<usize>) -> &mut str {
        &mut self[..][index]
    }
}

#[cfg(test)]
mod tests {
    #[cfg(alloc)]
    use alloc::borrow::ToOwned;
    #[cfg(alloc)]
    use alloc::vec;
    use core::ops::IndexMut;
    #[cfg(std)]
    use std::ffi::CStr;
    use RangeFromExclusive;
    use RangeFromExclusiveToExclusive;
    use RangeFromExclusiveToInclusive;

    
    #[test]
    fn range_from_exclusive_index_slice() {
        assert_eq!([0, 1, 2, 3, 4][RangeFromExclusive { start: 1 }], [2, 3, 4]);
        assert_eq!([0, 1, 2, 3, 4][RangeFromExclusive { start: 4 }], []);
    }

    
    #[test]
    #[should_panic]
    fn range_from_exclusive_index_slice_out_of_bounds() {
        let _ = [0, 1, 2, 3, 4][RangeFromExclusive { start: 5 }];
    }

    
    #[test]
    #[should_panic]
    fn range_from_exclusive_index_slice_from_max() {
        let _ = [0, 1, 2, 3, 4][RangeFromExclusive {
            start: core::usize::MAX,
        }];
    }

    
    #[test]
    fn range_from_exclusive_index_mut_slice() {
        let mut slice = [0, 1, 2, 3, 4];

        slice[RangeFromExclusive { start: 1 }][0] = 5;

        assert_eq!(slice, [0, 1, 5, 3, 4]);
        assert_eq!(slice.index_mut(RangeFromExclusive { start: 4 }), []);
    }

    
    #[test]
    #[should_panic]
    fn range_from_exclusive_index_mut_slice_out_of_bounds() {
        let _ = [0, 1, 2, 3, 4].index_mut(RangeFromExclusive { start: 5 });
    }

    
    #[test]
    #[should_panic]
    fn range_from_exclusive_index_mut_slice_from_max() {
        let _ = [0, 1, 2, 3, 4].index_mut(RangeFromExclusive {
            start: core::usize::MAX,
        });
    }

    
    #[test]
    fn range_from_exclusive_index_str() {
        assert_eq!(&"abcde"[RangeFromExclusive { start: 1 }], "cde");
        assert_eq!(&"abcde"[RangeFromExclusive { start: 4 }], "");
    }

    
    #[test]
    #[should_panic]
    fn range_from_exclusive_index_str_out_of_bounds() {
        let _ = "abcde"[RangeFromExclusive { start: 5 }];
    }

    
    #[test]
    #[should_panic]
    fn range_from_exclusive_index_str_from_max() {
        let _ = "abcde"[RangeFromExclusive {
            start: core::usize::MAX,
        }];
    }

    #[cfg(alloc)]
    #[test]
    fn range_from_exclusive_index_mut_str() {
        let mut s = "abcde".to_owned();
        let mut_s = s.as_mut_str();

        mut_s[RangeFromExclusive { start: 1 }].make_ascii_uppercase();

        assert_eq!(mut_s, "abCDE");
        assert_eq!(mut_s.index_mut(RangeFromExclusive { start: 4 }), "");
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_index_mut_str_out_of_bounds() {
        let _ = "abcde"
            .to_owned()
            .as_mut_str()
            .index_mut(RangeFromExclusive { start: 5 });
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_index_mut_str_from_max() {
        let _ = "abcde"
            .to_owned()
            .as_mut_str()
            .index_mut(RangeFromExclusive {
                start: core::usize::MAX,
            });
    }

    #[cfg(alloc)]
    #[test]
    fn range_from_exclusive_index_string() {
        assert_eq!(&"abcde".to_owned()[RangeFromExclusive { start: 1 }], "cde");
        assert_eq!(&"abcde".to_owned()[RangeFromExclusive { start: 4 }], "");
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_index_string_out_of_bounds() {
        let _ = "abcde".to_owned()[RangeFromExclusive { start: 5 }];
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_index_string_max() {
        let _ = "abcde".to_owned()[RangeFromExclusive {
            start: core::usize::MAX,
        }];
    }

    #[cfg(alloc)]
    #[test]
    fn range_from_exclusive_index_mut_string() {
        let mut s = "abcde".to_owned();

        s[RangeFromExclusive { start: 1 }].make_ascii_uppercase();

        assert_eq!(s, "abCDE");
        assert_eq!(s.index_mut(RangeFromExclusive { start: 4 }), "");
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_index_mut_string_out_of_bounds() {
        let _ = "abcde"
            .to_owned()
            .index_mut(RangeFromExclusive { start: 5 });
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_index_mut_string_from_max() {
        let _ = "abcde".to_owned().index_mut(RangeFromExclusive {
            start: core::usize::MAX,
        });
    }

    #[cfg(alloc)]
    #[test]
    fn range_from_exclusive_index_vec() {
        assert_eq!(
            vec![0, 1, 2, 3, 4][RangeFromExclusive { start: 1 }],
            [2, 3, 4]
        );
        assert_eq!(vec![0, 1, 2, 3, 4][RangeFromExclusive { start: 4 }], []);
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_index_vec_out_of_bounds() {
        let _ = vec![0, 1, 2, 3, 4][RangeFromExclusive { start: 5 }];
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_index_vec_from_max() {
        let _ = vec![0, 1, 2, 3, 4][RangeFromExclusive {
            start: core::usize::MAX,
        }];
    }

    #[cfg(alloc)]
    #[test]
    fn range_from_exclusive_index_mut_vec() {
        let mut v = vec![0, 1, 2, 3, 4];

        v[RangeFromExclusive { start: 1 }][0] = 5;

        assert_eq!(v, [0, 1, 5, 3, 4]);
        assert_eq!(v.index_mut(RangeFromExclusive { start: 4 }), []);
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_index_mut_vec_out_of_bounds() {
        let _ = vec![0, 1, 2, 3, 4].index_mut(RangeFromExclusive { start: 5 });
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_index_mut_vec_from_max() {
        let _ = vec![0, 1, 2, 3, 4].index_mut(RangeFromExclusive {
            start: core::usize::MAX,
        });
    }

    #[cfg(std)]
    #[test]
    fn range_from_exclusive_index_cstr() {
        assert_eq!(
            &CStr::from_bytes_with_nul(b"abcde\0").unwrap()[RangeFromExclusive { start: 1 }],
            CStr::from_bytes_with_nul(b"cde\0").unwrap()
        );
        assert_eq!(
            &CStr::from_bytes_with_nul(b"abcde\0").unwrap()[RangeFromExclusive { start: 4 }],
            CStr::from_bytes_with_nul(b"\0").unwrap()
        );
    }

    #[cfg(std)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_index_cstr_out_of_bounds() {
        let _ = CStr::from_bytes_with_nul(b"abcde\0").unwrap()[RangeFromExclusive { start: 5 }];
    }

    #[cfg(std)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_index_cstr_from_max() {
        let _ = CStr::from_bytes_with_nul(b"abcde\0").unwrap()[RangeFromExclusive {
            start: core::usize::MAX,
        }];
    }

    
    #[test]
    fn range_from_exclusive_to_inclusive_index_slice() {
        assert_eq!(
            [0, 1, 2, 3, 4][RangeFromExclusiveToInclusive { start: 1, end: 3 }],
            [2, 3]
        );
        assert_eq!(
            [0, 1, 2, 3, 4][RangeFromExclusiveToInclusive { start: 4, end: 4 }],
            []
        );
        assert_eq!(
            [0, 1, 2, 3, 4][RangeFromExclusiveToInclusive { start: 0, end: 0 }],
            []
        );
    }

    
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_slice_partially_out_of_bounds() {
        let _ = [0, 1, 2, 3, 4][RangeFromExclusiveToInclusive { start: 3, end: 5 }];
    }

    
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_slice_fully_out_of_bounds() {
        let _ = [0, 1, 2, 3, 4][RangeFromExclusiveToInclusive { start: 5, end: 7 }];
    }

    
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_slice_from_max() {
        let _ = [0, 1, 2, 3, 4][RangeFromExclusiveToInclusive {
            start: core::usize::MAX,
            end: 3,
        }];
    }

    
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_slice_to_max() {
        let _ = [0, 1, 2, 3, 4][RangeFromExclusiveToInclusive {
            start: 1,
            end: core::usize::MAX,
        }];
    }

    
    #[test]
    fn range_from_exclusive_to_inclusive_index_mut_slice() {
        let mut slice = [0, 1, 2, 3, 4];

        slice[RangeFromExclusiveToInclusive { start: 1, end: 3 }][0] = 5;

        assert_eq!(slice, [0, 1, 5, 3, 4]);
        assert_eq!(
            slice.index_mut(RangeFromExclusiveToInclusive { start: 4, end: 4 }),
            []
        );
        assert_eq!(
            slice.index_mut(RangeFromExclusiveToInclusive { start: 0, end: 0 }),
            []
        );
    }

    
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_mut_slice_partially_out_of_bounds() {
        let _ = [0, 1, 2, 3, 4].index_mut(RangeFromExclusiveToInclusive { start: 3, end: 5 });
    }

    
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_mut_slice_fully_out_of_bounds() {
        let _ = [0, 1, 2, 3, 4].index_mut(RangeFromExclusiveToInclusive { start: 5, end: 7 });
    }

    
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_mut_slice_from_max() {
        let _ = [0, 1, 2, 3, 4].index_mut(RangeFromExclusiveToInclusive {
            start: core::usize::MAX,
            end: 3,
        });
    }

    
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_mut_slice_to_max() {
        let _ = [0, 1, 2, 3, 4].index_mut(RangeFromExclusiveToInclusive {
            start: 1,
            end: core::usize::MAX,
        });
    }

    
    #[test]
    fn range_from_exclusive_to_inclusive_index_str() {
        assert_eq!(
            &"abcde"[RangeFromExclusiveToInclusive { start: 1, end: 3 }],
            "cd"
        );
        assert_eq!(
            &"abcde"[RangeFromExclusiveToInclusive { start: 4, end: 4 }],
            ""
        );
        assert_eq!(
            &"abcde"[RangeFromExclusiveToInclusive { start: 0, end: 0 }],
            ""
        );
    }

    
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_str_partially_out_of_bounds() {
        let _ = "abcde"[RangeFromExclusiveToInclusive { start: 3, end: 5 }];
    }

    
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_str_fully_out_of_bounds() {
        let _ = "abcde"[RangeFromExclusiveToInclusive { start: 5, end: 7 }];
    }

    
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_str_from_max() {
        let _ = "abcde"[RangeFromExclusiveToInclusive {
            start: core::usize::MAX,
            end: 3,
        }];
    }

    
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_str_to_max() {
        let _ = "abcde"[RangeFromExclusiveToInclusive {
            start: 1,
            end: core::usize::MAX,
        }];
    }

    #[cfg(alloc)]
    #[test]
    fn range_from_exclusive_to_inclusive_index_mut_str() {
        let mut s = "abcde".to_owned();
        let mut_s = s.as_mut_str();

        mut_s[RangeFromExclusiveToInclusive { start: 1, end: 3 }].make_ascii_uppercase();

        assert_eq!(mut_s, "abCDe");
        assert_eq!(
            mut_s.index_mut(RangeFromExclusiveToInclusive { start: 4, end: 4 }),
            ""
        );
        assert_eq!(
            mut_s.index_mut(RangeFromExclusiveToInclusive { start: 0, end: 0 }),
            ""
        );
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_mut_str_partially_out_of_bounds() {
        let _ = "abcde"
            .to_owned()
            .as_mut_str()
            .index_mut(RangeFromExclusiveToInclusive { start: 3, end: 5 });
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_mut_str_fully_out_of_bounds() {
        let _ = "abcde"
            .to_owned()
            .as_mut_str()
            .index_mut(RangeFromExclusiveToInclusive { start: 5, end: 7 });
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_mut_str_from_max() {
        let _ = "abcde"
            .to_owned()
            .as_mut_str()
            .index_mut(RangeFromExclusiveToInclusive {
                start: core::usize::MAX,
                end: 3,
            });
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_mut_str_to_max() {
        let _ = "abcde"
            .to_owned()
            .as_mut_str()
            .index_mut(RangeFromExclusiveToInclusive {
                start: 1,
                end: core::usize::MAX,
            });
    }

    #[cfg(alloc)]
    #[test]
    fn range_from_exclusive_to_inclusive_index_string() {
        assert_eq!(
            &"abcde".to_owned()[RangeFromExclusiveToInclusive { start: 1, end: 3 }],
            "cd"
        );
        assert_eq!(
            &"abcde".to_owned()[RangeFromExclusiveToInclusive { start: 4, end: 4 }],
            ""
        );
        assert_eq!(
            &"abcde".to_owned()[RangeFromExclusiveToInclusive { start: 0, end: 0 }],
            ""
        );
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_string_partially_out_of_bounds() {
        let _ = "abcde".to_owned()[RangeFromExclusiveToInclusive { start: 3, end: 5 }];
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_string_fully_out_of_bounds() {
        let _ = "abcde".to_owned()[RangeFromExclusiveToInclusive { start: 5, end: 7 }];
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_string_from_max() {
        let _ = "abcde".to_owned()[RangeFromExclusiveToInclusive {
            start: core::usize::MAX,
            end: 3,
        }];
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_string_to_max() {
        let _ = "abcde".to_owned()[RangeFromExclusiveToInclusive {
            start: 1,
            end: core::usize::MAX,
        }];
    }

    #[cfg(alloc)]
    #[test]
    fn range_from_exclusive_to_inclusive_index_mut_string() {
        let mut s = "abcde".to_owned();

        s[RangeFromExclusiveToInclusive { start: 1, end: 3 }].make_ascii_uppercase();

        assert_eq!(s, "abCDe");
        assert_eq!(
            s.index_mut(RangeFromExclusiveToInclusive { start: 4, end: 4 }),
            ""
        );
        assert_eq!(
            s.index_mut(RangeFromExclusiveToInclusive { start: 0, end: 0 }),
            ""
        );
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_mut_string_partially_out_of_bounds() {
        let _ = "abcde"
            .to_owned()
            .index_mut(RangeFromExclusiveToInclusive { start: 3, end: 5 });
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_mut_string_fully_out_of_bounds() {
        let _ = "abcde"
            .to_owned()
            .index_mut(RangeFromExclusiveToInclusive { start: 5, end: 7 });
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_mut_string_from_max() {
        let _ = "abcde".to_owned().index_mut(RangeFromExclusiveToInclusive {
            start: core::usize::MAX,
            end: 3,
        });
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_mut_string_to_max() {
        let _ = "abcde".to_owned().index_mut(RangeFromExclusiveToInclusive {
            start: 1,
            end: core::usize::MAX,
        });
    }

    #[cfg(alloc)]
    #[test]
    fn range_from_exclusive_to_inclusive_index_vec() {
        assert_eq!(
            vec![0, 1, 2, 3, 4][RangeFromExclusiveToInclusive { start: 1, end: 3 }],
            [2, 3]
        );
        assert_eq!(
            vec![0, 1, 2, 3, 4][RangeFromExclusiveToInclusive { start: 4, end: 4 }],
            []
        );
        assert_eq!(
            vec![0, 1, 2, 3, 4][RangeFromExclusiveToInclusive { start: 0, end: 0 }],
            []
        );
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_vec_partially_out_of_bounds() {
        let _ = vec![0, 1, 2, 3, 4][RangeFromExclusiveToInclusive { start: 3, end: 5 }];
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_vec_fully_out_of_bounds() {
        let _ = vec![0, 1, 2, 3, 4][RangeFromExclusiveToInclusive { start: 5, end: 7 }];
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_vec_from_max() {
        let _ = vec![0, 1, 2, 3, 4][RangeFromExclusiveToInclusive {
            start: core::usize::MAX,
            end: 3,
        }];
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_vec_to_max() {
        let _ = vec![0, 1, 2, 3, 4][RangeFromExclusiveToInclusive {
            start: 1,
            end: core::usize::MAX,
        }];
    }

    #[cfg(alloc)]
    #[test]
    fn range_from_exclusive_to_inclusive_index_mut_vec() {
        let mut v = vec![0, 1, 2, 3, 4];

        v[RangeFromExclusiveToInclusive { start: 1, end: 3 }][0] = 5;

        assert_eq!(v, [0, 1, 5, 3, 4]);
        assert_eq!(
            v.index_mut(RangeFromExclusiveToInclusive { start: 4, end: 4 }),
            []
        );
        assert_eq!(
            v.index_mut(RangeFromExclusiveToInclusive { start: 0, end: 0 }),
            []
        );
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_mut_vec_partially_out_of_bounds() {
        let _ =
            vec![0, 1, 2, 3, 4].index_mut(RangeFromExclusiveToInclusive { start: 3, end: 5 });
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_mut_vec_fully_out_of_bounds() {
        let _ =
            vec![0, 1, 2, 3, 4].index_mut(RangeFromExclusiveToInclusive { start: 5, end: 7 });
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_mut_vec_from_max() {
        let _ = vec![0, 1, 2, 3, 4].index_mut(RangeFromExclusiveToInclusive {
            start: core::usize::MAX,
            end: 3,
        });
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_inclusive_index_mut_vec_to_max() {
        let _ = vec![0, 1, 2, 3, 4].index_mut(RangeFromExclusiveToInclusive {
            start: 1,
            end: core::usize::MAX,
        });
    }

    
    #[test]
    fn range_from_exclusive_to_exclusive_index_slice() {
        assert_eq!(
            [0, 1, 2, 3, 4][RangeFromExclusiveToExclusive { start: 1, end: 3 }],
            [2]
        );
        assert_eq!(
            [0, 1, 2, 3, 4][RangeFromExclusiveToExclusive { start: 4, end: 5 }],
            []
        );
        assert_eq!(
            [0, 1, 2, 3, 4][RangeFromExclusiveToExclusive { start: 0, end: 1 }],
            []
        );
    }

    
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_exclusive_index_slice_partially_out_of_bounds() {
        let _ = [0, 1, 2, 3, 4][RangeFromExclusiveToExclusive { start: 3, end: 6 }];
    }

    
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_exclusive_index_slice_fully_out_of_bounds() {
        let _ = [0, 1, 2, 3, 4][RangeFromExclusiveToExclusive { start: 5, end: 7 }];
    }

    
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_exclusive_index_slice_from_max() {
        let _ = [0, 1, 2, 3, 4][RangeFromExclusiveToExclusive {
            start: core::usize::MAX,
            end: 3,
        }];
    }

    
    #[test]
    fn range_from_exclusive_to_exclusive_index_mut_slice() {
        let mut slice = [0, 1, 2, 3, 4];

        slice[RangeFromExclusiveToExclusive { start: 1, end: 3 }][0] = 5;

        assert_eq!(slice, [0, 1, 5, 3, 4]);
        assert_eq!(
            slice.index_mut(RangeFromExclusiveToExclusive { start: 4, end: 5 }),
            []
        );
        assert_eq!(
            slice.index_mut(RangeFromExclusiveToExclusive { start: 0, end: 1 }),
            []
        );
    }

    
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_exclusive_index_mut_slice_partially_out_of_bounds() {
        let _ = [0, 1, 2, 3, 4].index_mut(RangeFromExclusiveToExclusive { start: 3, end: 6 });
    }

    
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_exclusive_index_mut_slice_fully_out_of_bounds() {
        let _ = [0, 1, 2, 3, 4].index_mut(RangeFromExclusiveToExclusive { start: 5, end: 7 });
    }

    
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_exclusive_index_mut_slice_from_max() {
        let _ = [0, 1, 2, 3, 4].index_mut(RangeFromExclusiveToExclusive {
            start: core::usize::MAX,
            end: 3,
        });
    }

    
    #[test]
    fn range_from_exclusive_to_exclusive_index_str() {
        assert_eq!(
            &"abcde"[RangeFromExclusiveToExclusive { start: 1, end: 3 }],
            "c"
        );
        assert_eq!(
            &"abcde"[RangeFromExclusiveToExclusive { start: 4, end: 5 }],
            ""
        );
        assert_eq!(
            &"abcde"[RangeFromExclusiveToExclusive { start: 0, end: 1 }],
            ""
        );
    }

    
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_exclusive_index_str_partially_out_of_bounds() {
        let _ = "abcde"[RangeFromExclusiveToExclusive { start: 3, end: 6 }];
    }

    
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_exclusive_index_str_fully_out_of_bounds() {
        let _ = "abcde"[RangeFromExclusiveToExclusive { start: 5, end: 7 }];
    }

    
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_exclusive_index_str_from_max() {
        let _ = "abcde"[RangeFromExclusiveToExclusive {
            start: core::usize::MAX,
            end: 3,
        }];
    }

    #[cfg(alloc)]
    #[test]
    fn range_from_exclusive_to_exclusive_index_mut_str() {
        let mut s = "abcde".to_owned();
        let mut_s = s.as_mut_str();

        mut_s[RangeFromExclusiveToExclusive { start: 1, end: 3 }].make_ascii_uppercase();

        assert_eq!(mut_s, "abCde");
        assert_eq!(
            mut_s.index_mut(RangeFromExclusiveToExclusive { start: 4, end: 5 }),
            ""
        );
        assert_eq!(
            mut_s.index_mut(RangeFromExclusiveToExclusive { start: 0, end: 1 }),
            ""
        );
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_exclusive_index_mut_str_partially_out_of_bounds() {
        let _ = "abcde"
            .to_owned()
            .as_mut_str()
            .index_mut(RangeFromExclusiveToExclusive { start: 3, end: 6 });
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_exclusive_index_mut_str_fully_out_of_bounds() {
        let _ = "abcde"
            .to_owned()
            .as_mut_str()
            .index_mut(RangeFromExclusiveToExclusive { start: 5, end: 7 });
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_exclusive_index_mut_str_from_max() {
        let _ = "abcde"
            .to_owned()
            .as_mut_str()
            .index_mut(RangeFromExclusiveToExclusive {
                start: core::usize::MAX,
                end: 3,
            });
    }

    #[cfg(alloc)]
    #[test]
    fn range_from_exclusive_to_exclusive_index_string() {
        assert_eq!(
            &"abcde".to_owned()[RangeFromExclusiveToExclusive { start: 1, end: 3 }],
            "c"
        );
        assert_eq!(
            &"abcde".to_owned()[RangeFromExclusiveToExclusive { start: 4, end: 5 }],
            ""
        );
        assert_eq!(
            &"abcde".to_owned()[RangeFromExclusiveToExclusive { start: 0, end: 1 }],
            ""
        );
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_exclusive_index_string_partially_out_of_bounds() {
        let _ = "abcde".to_owned()[RangeFromExclusiveToExclusive { start: 3, end: 6 }];
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_exclusive_index_string_fully_out_of_bounds() {
        let _ = "abcde".to_owned()[RangeFromExclusiveToExclusive { start: 5, end: 7 }];
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_exclusive_index_string_from_max() {
        let _ = "abcde".to_owned()[RangeFromExclusiveToExclusive {
            start: core::usize::MAX,
            end: 3,
        }];
    }

    #[cfg(alloc)]
    #[test]
    fn range_from_exclusive_to_exclusive_index_mut_string() {
        let mut s = "abcde".to_owned();

        s[RangeFromExclusiveToExclusive { start: 1, end: 3 }].make_ascii_uppercase();

        assert_eq!(s, "abCde");
        assert_eq!(
            s.index_mut(RangeFromExclusiveToExclusive { start: 4, end: 5 }),
            ""
        );
        assert_eq!(
            s.index_mut(RangeFromExclusiveToExclusive { start: 0, end: 1 }),
            ""
        );
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_exclusive_index_mut_string_partially_out_of_bounds() {
        let _ = "abcde"
            .to_owned()
            .index_mut(RangeFromExclusiveToExclusive { start: 3, end: 6 });
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_exclusive_index_mut_string_fully_out_of_bounds() {
        let _ = "abcde"
            .to_owned()
            .index_mut(RangeFromExclusiveToExclusive { start: 5, end: 7 });
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_exclusive_index_mut_string_from_max() {
        let _ = "abcde".to_owned().index_mut(RangeFromExclusiveToExclusive {
            start: core::usize::MAX,
            end: 3,
        });
    }

    #[cfg(alloc)]
    #[test]
    fn range_from_exclusive_to_exclusive_index_vec() {
        assert_eq!(
            vec![0, 1, 2, 3, 4][RangeFromExclusiveToExclusive { start: 1, end: 3 }],
            [2]
        );
        assert_eq!(
            vec![0, 1, 2, 3, 4][RangeFromExclusiveToExclusive { start: 4, end: 5 }],
            []
        );
        assert_eq!(
            vec![0, 1, 2, 3, 4][RangeFromExclusiveToExclusive { start: 0, end: 1 }],
            []
        );
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_exclusive_index_vec_partially_out_of_bounds() {
        let _ = vec![0, 1, 2, 3, 4][RangeFromExclusiveToExclusive { start: 3, end: 6 }];
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_exclusive_index_vec_fully_out_of_bounds() {
        let _ = vec![0, 1, 2, 3, 4][RangeFromExclusiveToExclusive { start: 5, end: 7 }];
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_exclusive_index_vec_from_max() {
        let _ = vec![0, 1, 2, 3, 4][RangeFromExclusiveToExclusive {
            start: core::usize::MAX,
            end: 3,
        }];
    }

    #[cfg(alloc)]
    #[test]
    fn range_from_exclusive_to_exclusive_index_mut_vec() {
        let mut v = vec![0, 1, 2, 3, 4];

        v[RangeFromExclusiveToExclusive { start: 1, end: 3 }][0] = 5;

        assert_eq!(v, [0, 1, 5, 3, 4]);
        assert_eq!(
            v.index_mut(RangeFromExclusiveToExclusive { start: 4, end: 5 }),
            []
        );
        assert_eq!(
            v.index_mut(RangeFromExclusiveToExclusive { start: 0, end: 1 }),
            []
        );
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_exclusive_index_mut_vec_partially_out_of_bounds() {
        let _ =
            vec![0, 1, 2, 3, 4].index_mut(RangeFromExclusiveToExclusive { start: 3, end: 6 });
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_exclusive_index_mut_vec_fully_out_of_bounds() {
        let _ =
            vec![0, 1, 2, 3, 4].index_mut(RangeFromExclusiveToExclusive { start: 5, end: 7 });
    }

    #[cfg(alloc)]
    #[test]
    #[should_panic]
    fn range_from_exclusive_to_exclusive_index_mut_vec_from_max() {
        let _ = vec![0, 1, 2, 3, 4].index_mut(RangeFromExclusiveToExclusive {
            start: core::usize::MAX,
            end: 3,
        });
    }
}
