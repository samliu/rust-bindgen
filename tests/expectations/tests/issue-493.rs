/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl <T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self { __BindgenUnionField(::std::marker::PhantomData) }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T { ::std::mem::transmute(self) }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T { ::std::mem::transmute(self) }
}
impl <T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self { Self::new() }
}
impl <T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self { Self::new() }
}
impl <T> ::std::marker::Copy for __BindgenUnionField<T> { }
impl <T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct basic_string<_CharT, _Traits, _Allocator> {
    pub _address: u8,
    pub _phantom_0: ::std::marker::PhantomData<_CharT>,
    pub _phantom_1: ::std::marker::PhantomData<_Traits>,
    pub _phantom_2: ::std::marker::PhantomData<_Allocator>,
}
pub type basic_string_size_type = ::std::os::raw::c_ulonglong;
pub type basic_string_value_type = ::std::os::raw::c_char;
pub type basic_string_pointer = *mut basic_string_value_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct basic_string___long<_CharT, _Traits, _Allocator> {
    pub __cap_: basic_string_size_type,
    pub __size_: basic_string_size_type,
    pub __data_: basic_string_pointer,
    pub _phantom_0: ::std::marker::PhantomData<_CharT>,
    pub _phantom_1: ::std::marker::PhantomData<_Traits>,
    pub _phantom_2: ::std::marker::PhantomData<_Allocator>,
}
impl <_CharT, _Traits, _Allocator> Default for
 basic_string___long<_CharT, _Traits, _Allocator> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub const basic_string___min_cap: basic_string__bindgen_ty_1 =
    basic_string__bindgen_ty_1::__min_cap;
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum basic_string__bindgen_ty_1 { __min_cap = 0, }
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct basic_string___short<_CharT, _Traits, _Allocator> {
    pub __bindgen_anon_1: basic_string___short__bindgen_ty_1<_CharT, _Traits,
                                                             _Allocator>,
    pub __data_: *mut basic_string_value_type,
    pub _phantom_0: ::std::marker::PhantomData<_CharT>,
    pub _phantom_1: ::std::marker::PhantomData<_Traits>,
    pub _phantom_2: ::std::marker::PhantomData<_Allocator>,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct basic_string___short__bindgen_ty_1<_CharT, _Traits, _Allocator> {
    pub __size_: __BindgenUnionField<::std::os::raw::c_uchar>,
    pub __lx: __BindgenUnionField<basic_string_value_type>,
    pub bindgen_union_field: u8,
    pub _phantom_0: ::std::marker::PhantomData<_CharT>,
    pub _phantom_1: ::std::marker::PhantomData<_Traits>,
    pub _phantom_2: ::std::marker::PhantomData<_Allocator>,
}
impl <_CharT, _Traits, _Allocator> Default for
 basic_string___short<_CharT, _Traits, _Allocator> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct basic_string___ulx<_CharT, _Traits, _Allocator> {
    pub __lx: __BindgenUnionField<basic_string___long<_CharT, _Traits,
                                                      _Allocator>>,
    pub __lxx: __BindgenUnionField<basic_string___short<_CharT, _Traits,
                                                        _Allocator>>,
    pub bindgen_union_field: [u8; 0usize],
    pub _phantom_0: ::std::marker::PhantomData<_CharT>,
    pub _phantom_1: ::std::marker::PhantomData<_Traits>,
    pub _phantom_2: ::std::marker::PhantomData<_Allocator>,
}
impl <_CharT, _Traits, _Allocator> Default for
 basic_string___ulx<_CharT, _Traits, _Allocator> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub const basic_string___n_words: basic_string__bindgen_ty_2 =
    basic_string__bindgen_ty_2::__n_words;
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum basic_string__bindgen_ty_2 { __n_words = 0, }
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct basic_string___raw<_CharT, _Traits, _Allocator> {
    pub __words: *mut basic_string_size_type,
    pub _phantom_0: ::std::marker::PhantomData<_CharT>,
    pub _phantom_1: ::std::marker::PhantomData<_Traits>,
    pub _phantom_2: ::std::marker::PhantomData<_Allocator>,
}
impl <_CharT, _Traits, _Allocator> Default for
 basic_string___raw<_CharT, _Traits, _Allocator> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct basic_string___rep<_CharT, _Traits, _Allocator> {
    pub __bindgen_anon_1: basic_string___rep__bindgen_ty_1<_CharT, _Traits,
                                                           _Allocator>,
    pub _phantom_0: ::std::marker::PhantomData<_CharT>,
    pub _phantom_1: ::std::marker::PhantomData<_Traits>,
    pub _phantom_2: ::std::marker::PhantomData<_Allocator>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct basic_string___rep__bindgen_ty_1<_CharT, _Traits, _Allocator> {
    pub __l: __BindgenUnionField<basic_string___long<_CharT, _Traits,
                                                     _Allocator>>,
    pub __s: __BindgenUnionField<basic_string___short<_CharT, _Traits,
                                                      _Allocator>>,
    pub __r: __BindgenUnionField<basic_string___raw<_CharT, _Traits,
                                                    _Allocator>>,
    pub bindgen_union_field: [u8; 0usize],
    pub _phantom_0: ::std::marker::PhantomData<_CharT>,
    pub _phantom_1: ::std::marker::PhantomData<_Traits>,
    pub _phantom_2: ::std::marker::PhantomData<_Allocator>,
}
impl <_CharT, _Traits, _Allocator> Default for
 basic_string___rep__bindgen_ty_1<_CharT, _Traits, _Allocator> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
impl <_CharT, _Traits, _Allocator> Default for
 basic_string___rep<_CharT, _Traits, _Allocator> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
impl <_CharT, _Traits, _Allocator> Default for
 basic_string<_CharT, _Traits, _Allocator> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
