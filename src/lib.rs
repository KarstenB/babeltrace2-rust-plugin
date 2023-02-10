// Copyright 2023 - 2023, Karsten Becker and the rust babeltrace2-plugin contributors
// SPDX-License-Identifier: GPL-2.0-or-later
pub mod bt2;
use std::ffi::CStr;
use std::mem::MaybeUninit;

use bt2::binding::*;
use bt2::*;

pub struct BtVersion {
}
impl BtVersion {
    pub fn get_major() -> u32 {
        unsafe {bt_version_get_major()}
    }
    pub fn get_minor() -> u32 {
        unsafe {bt_version_get_minor()}
    }
    pub fn get_patch() -> u32 {
        unsafe {bt_version_get_patch()}
    }
    pub fn get_development_stage() -> &'static CStr {
        unsafe {CStr::from_ptr(bt_version_get_development_stage())}
    }
    pub fn get_vcs_revision_description() -> &'static CStr {
        unsafe {CStr::from_ptr(bt_version_get_vcs_revision_description())}
    }
    pub fn get_name() -> &'static CStr {
        unsafe {CStr::from_ptr(bt_version_get_name())}
    }
    pub fn get_name_description() -> &'static CStr {
        unsafe {CStr::from_ptr(bt_version_get_name_description())}
    }
    pub fn get_extra_name() -> &'static CStr {
        unsafe {CStr::from_ptr(bt_version_get_extra_name())}
    }
    pub fn get_extra_description() -> &'static CStr {
        unsafe {CStr::from_ptr(bt_version_get_extra_description())}
    }
    pub fn get_extra_patch_names() -> &'static CStr {
        unsafe {CStr::from_ptr(bt_version_get_extra_patch_names())}
    }
}

pub fn iterator_to_vec(
    iter: &mut BtMessageIterator,
) -> Result<Vec<BtMessageConst>, BtMessageIteratorNextStatus> {
    /* Consume a batch of messages from the upstream message iterator */
    let mut messages: MaybeUninit<*mut *const bt_message> = MaybeUninit::uninit();
    let mut count: u64 = 0;
    let next_status = unsafe {iter.next(messages.as_mut_ptr(), &mut count)};

    if next_status != BtMessageIteratorNextStatus::Ok {
        return Err(next_status);
    }
    let msg_out: *mut *const bt_message = unsafe { *messages.as_mut_ptr() };
    let msgs: &[*const bt_message] = unsafe { std::slice::from_raw_parts(msg_out, count as usize) };
    /* For each consumed message */
    let mut result = Vec::new();
    for msg in msgs {
        result.push(BtMessageConst::from_ptr(*msg));
    }
    Ok(result)
}

impl From<BtMessageIteratorNextStatus> for BtComponentClassSinkConsumeMethodStatus {
    fn from(item: BtMessageIteratorNextStatus) -> Self {
        match item {
            BtMessageIteratorNextStatus::Ok => BtComponentClassSinkConsumeMethodStatus::Ok,
            BtMessageIteratorNextStatus::End => BtComponentClassSinkConsumeMethodStatus::End,
            BtMessageIteratorNextStatus::Again => BtComponentClassSinkConsumeMethodStatus::Again,
            BtMessageIteratorNextStatus::MemoryError => {
                BtComponentClassSinkConsumeMethodStatus::MemoryError
            }
            BtMessageIteratorNextStatus::Error => BtComponentClassSinkConsumeMethodStatus::Error,
        }
    }
}

pub trait ToSelfComponent {
    fn to_self_component(&mut self) -> BtSelfComponent;
}

impl ToSelfComponent for BtSelfComponentSink {
    fn to_self_component(&mut self) -> BtSelfComponent {
        self.as_self_component_inline()
    }
}

impl ToSelfComponent for BtSelfComponentSource {
    fn to_self_component(&mut self) -> BtSelfComponent {
        self.as_self_component_inline()
    }
}

impl ToSelfComponent for BtSelfComponentFilter {
    fn to_self_component(&mut self) -> BtSelfComponent {
        self.as_self_component_inline()
    }
}

pub fn set_boxed_data<T>(comp: &mut dyn ToSelfComponent, data:Box<T>) {
    unsafe {comp.to_self_component().set_data(Box::into_raw(data) as *mut std::ffi::c_void)}
}

pub fn get_boxed_data<T>(comp: &mut dyn ToSelfComponent) -> Box<T> {
    unsafe { Box::from_raw(comp.to_self_component().get_data() as *mut T) }
}

pub fn get_scoped_boxed_data<T, F, R>(comp: &mut dyn ToSelfComponent, mut scope: F) -> R
where
    F: FnMut(&mut Box<T>) -> R,
{
    let mut box_data: Box<T> = get_boxed_data(comp);
    let res = scope(&mut box_data);
    Box::leak(box_data);
    res
}

pub fn get_iterator(comp: &BtSelfComponentSink, in_port: &BtSelfComponentPortInput) -> BtMessageIterator{
    let mut iter: MaybeUninit<*mut bt_message_iterator> = MaybeUninit::uninit();
    unsafe {
    BtMessageIterator::create_from_sink_component(comp, in_port, iter.as_mut_ptr());
    BtMessageIterator::from_ptr(*iter.as_mut_ptr())}
}

pub fn drop_data(comp: &mut dyn ToSelfComponent){
    let _data = unsafe {Box::from_raw(comp.to_self_component().get_data() as *mut _)};
}

#[cfg(test)]
mod tests {
    mod code_gen;
    use std::{path::PathBuf, fs};

    use code_gen::{generate_bt_lib, to_camel_case};

    #[test]
    fn bindgen() {
        let p = PathBuf::from(concat!(env!("OUT_DIR"), "/bindings.rs"));
        assert!(p.exists());
        let out = PathBuf::from("src/bt2.rs");
        let doc = PathBuf::from("src/bt2_doc.rs");
        let doc_str=fs::read_to_string(doc).expect("Failed to read documentation file");
        let count = generate_bt_lib(&p, &out, &doc_str);
        assert_eq!(count.unwrap(), 656);
    }

    #[test]
    fn camel_case() {
        assert_eq!(to_camel_case(""), "");
        assert_eq!(to_camel_case("_"), "");
        assert_eq!(to_camel_case("x"), "X");
        assert_eq!(to_camel_case("x_"), "X");
        assert_eq!(to_camel_case("x__"), "X");
        assert_eq!(to_camel_case("__x_"), "X");
        assert_eq!(to_camel_case("a_b"), "AB");
        assert_eq!(to_camel_case("aBCD_efgh"), "AbcdEfgh");
        assert_eq!(to_camel_case("abcd_efgh_ijkl"), "AbcdEfghIjkl");
    }
}
