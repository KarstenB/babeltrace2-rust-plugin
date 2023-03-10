// Copyright 2023 - 2023, Karsten Becker and the rust babeltrace2-plugin contributors
// SPDX-License-Identifier: GPL-2.0-or-later
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeMap;
use std::io::{Result, Write};
use std::mem::size_of;
use std::{
    fs::{self, File},
    io::BufWriter,
    path::PathBuf,
};

pub fn generate_bt_lib(bindgen_file: &PathBuf, output_file: &PathBuf, header_comment:&str) -> Result<i32> {
    let mut count = 0;
    let text = fs::read_to_string(bindgen_file).expect("Failed to read file");
    let write_file = File::create(output_file).unwrap();
    let mut writer = BufWriter::new(&write_file);
    let mut enum_types: BTreeMap<String, EnumInfo> = BTreeMap::new();
    writeln!(writer, "{}", header_comment)?;
    writeln!(writer, "//This file is automatically generated")?;
    writeln!(writer, "#![allow(clippy::upper_case_acronyms)]")?;
    writeln!(writer, "#![allow(clippy::missing_safety_doc)]")?;
    writeln!(writer, "#![allow(non_upper_case_globals)]")?;
    writeln!(writer, "#![allow(non_camel_case_types)]")?;
    writeln!(writer, "#![allow(non_snake_case)]")?;
    writeln!(writer, "#![allow(dead_code)]")?;
    writeln!(writer, "use std::ffi::CStr;")?;
    writeln!(writer, "use num_derive::{{FromPrimitive, ToPrimitive}};")?;
    writeln!(
        writer,
        r#"pub mod binding {{
  include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
  include!("binding_additions.rs");
}}
use binding::*;
include!("bt2_additions.rs");
"#
    )?;
    generate_enum_types(&text, &mut enum_types, &mut writer)?;
    generate_structs_with_functions(&text, &mut count, &mut writer, &enum_types)?;

    Ok(count)
}

fn generate_enum_types(
    text: &str,
    result: &mut BTreeMap<String, EnumInfo>,
    writer: &mut BufWriter<&File>,
) -> Result<()> {
    lazy_static! {
        static ref TYPE_REGEX:Regex=Regex::new(r#"(?s)pub\s+type\s+([a-zA-Z0-9_-]+?)\s*=\s*([a-zA-Z0-9_\-:]+?);"#).expect("Failed to compile regex");
        static ref VALUE_REGEX:Regex=Regex::new(r#"(?s)pub\s+const\s+([a-zA-Z0-9_-]+?)\s*:\s*([a-zA-Z0-9_-]+?)\s*=\s*(.*?)\s*;"#).expect("Failed to compile regex");
    }
    for cap in TYPE_REGEX.captures_iter(text) {
        let name = cap.get(1).unwrap().as_str().to_string();
        let rust_type = cap.get(2).unwrap().as_str().to_string();
        if name.starts_with("bt") {
            result.insert(
                name.clone(),
                EnumInfo {
                    name,
                    rust_type,
                    ..Default::default()
                },
            );
        }
    }
    for cap in VALUE_REGEX.captures_iter(text) {
        let name = cap.get(1).unwrap().as_str().to_string();
        let rust_type = cap.get(2).unwrap().as_str();
        if rust_type.starts_with("bt") {
            let mut short_name = name[rust_type.len() + 1..].to_string();
            if short_name.starts_with(&rust_type.to_uppercase()) {
                short_name = short_name[rust_type.len() + 1..].to_string();
            }
            let e_value = EnumValue {
                name: short_name,
                value: name.to_string(),
            };
            result
                .get_mut(rust_type)
                .map(|f| f.add_value(e_value))
                .unwrap_or_else(|| println!("failed to find {} for {}", &rust_type, &name));
        }
    }
    //Ensuring the match below is true
    assert!(size_of::<::std::os::raw::c_int>() == 4);
    assert!(size_of::<::std::os::raw::c_uint>() == 4);
    assert!(size_of::<::std::os::raw::c_long>() == 8);
    assert!(size_of::<::std::os::raw::c_ulong>() == 8);
    let mut del_empty_enums: Vec<String> = Vec::new();
    for enums in result.values() {
        if enums.values.is_empty() {
            println!("Skipping empty enum {}", enums.name);
            del_empty_enums.push(enums.name.to_string());
            continue;
        }
        let orignal_type_name = &enums.name;
        let enum_name = to_camel_case(&enums.name);
        let rust_type = get_enum_primitive_type(enums);
        writeln!(
            writer,
            r#"
#[repr({rust_type})]
#[derive(FromPrimitive, ToPrimitive, Debug, PartialEq, PartialOrd, Copy, Clone)]
#[doc(alias = "{orignal_type_name}")]
/// Represents the {orignal_type_name} as rust enum
pub enum {enum_name} {{"#
        )?;
        for value in &enums.values {
            //We should find the original C name here and make a proper alias
            let value_name=&value.value;
            writeln!(writer, r#"  #[doc(alias = "{value_name}")]"#)?;
            writeln!(writer, r#"  /// Alias for {value_name}"#)?;
            writeln!(writer, r#"  {} = {},"#, to_camel_case(&value.name), &value.value)?;
        }
        writeln!(writer, "}}")?;
        writeln!(writer, r#"impl From< {1:} > for {0:} {{
  fn from(item: {1:}) -> Self {{
    num::FromPrimitive::from_{1:}(item).unwrap()
  }}
}}
impl From< {0:} > for {1:} {{
  fn from(item: {0:}) -> Self {{
    num::ToPrimitive::to_{1:}(&item).unwrap()
  }}
}}
"#, enum_name, rust_type)?;
    }
    for key in del_empty_enums {
        result.remove_entry(&key);
    }
    Ok(())
}

fn get_enum_primitive_type(enums: &EnumInfo) -> &str {
    match enums.rust_type.as_str() {
        "::std::os::raw::c_int" => "i32",
        "::std::os::raw::c_uint" => "u32",
        "::std::os::raw::c_long" => "i64",
        "::std::os::raw::c_ulong" => "u64",
        "i32" => "i32",
        "u32" => "u32",
        "i64" => "i64",
        "u64" => "u64",
        x => panic!("Don't know what to do with enum type {x}"),
    }
}

fn generate_structs_with_functions(
    text: &str,
    count: &mut i32,
    writer: &mut BufWriter<&File>,
    enum_types: &BTreeMap<String, EnumInfo>,
) -> Result<()> {
    lazy_static! {
        static ref FUNCTION_REGEX:Regex=Regex::new(r#"(?s)extern\s+?"C"\s+?\{\s*?pub\s+?fn\s+?(?P<fn_name>[a-zA-Z_\-0-9]*?)\s*?\((?P<first_arg>.*?)(?P<rem_arg>,.*?)?\)\s*?(?P<return>->\s*?[a-zA-Z_\-* 0-9:]+?)?;\s*?}"#).expect("Failed to compile regex");
        static ref STRUCTS_REGEX:Regex=Regex::new(r#"(?s)pub\s+struct\s+(bt_[a-zA-Z_\-0-9]*?)\s*\{"#).expect("Failed to compile regex");
    }

    let mut result: BTreeMap<String, TypeInfo> = BTreeMap::new();
    for cap in STRUCTS_REGEX.captures_iter(text) {
        let bt_name = cap.get(1).unwrap().as_str().to_string();
        let name = to_camel_case(bt_name.as_str());
        let ti = TypeInfo {
            name,
            bt_name: bt_name.to_string(),
            ..Default::default()
        };
        result.insert(bt_name.to_string(), ti);
    }
    for cap in FUNCTION_REGEX.captures_iter(text) {
        if handle_function_regex_match(cap, &mut result, enum_types) {
            *count += 1;
        }
    }
    rewrite_types(&mut result, enum_types);
    for ti in result.values() {
        generate_fun(writer, ti, true)?;
        generate_fun(writer, ti, false)?;
    }
    Ok(())
}

fn generate_fun(writer: &mut BufWriter<&File>, ti: &TypeInfo, do_const: bool) -> Result<bool> {
    if do_const {
        write!(
            writer,
            r#"/// Wraps all methods related to {1:}, but the pointer is const
pub struct {0:}Const {{
    ptr: *const {1:}
}}
impl {0:}Const {{
    /// Generate a {0:}Const pointing to null. This can be useful to allocate in arrays or vectors,
    /// but calling a function on it is checked with a debug_assert. In release this would case 
    /// a segmentation fault!
    pub fn empty() -> {0:}Const {{
        {0:}Const {{ ptr: std::ptr::null_mut() }}
    }}
    /// Generate a {0:}Const pointing to ptr. This is assumed to be a proper pointer obtained 
    /// from a lower-level API. Null pointer are not allowed and checked with an assert.
    pub fn from_ptr(ptr:*const {1:}) -> {0:}Const {{
        assert!(!ptr.is_null());
        {0:}Const {{ ptr }}
    }}
    /// Check if the stored pointer is a null pointer or not.
    pub fn is_empty(&mut self) -> bool {{
        self.ptr.is_null()
    }}
"#,
            ti.name, ti.bt_name
        )?;
    } else {
        write!(
            writer,
            r#"/// Wraps all methods related to {1:}
pub struct {0:} {{
    ptr: *mut {1:}
}}
impl {0:} {{
    /// Generate a {0:}Const pointing to null. This can be useful to allocate in arrays or vectors,
    /// but calling a function on it is checked with a debug_assert. In release this would case 
    /// a segmentation fault!
    pub fn empty() -> {0:} {{
        {0:} {{ ptr: std::ptr::null_mut() }}
    }}
    /// Generate a {0:}Const pointing to ptr. This is assumed to be a proper pointer obtained 
    /// from a lower-level API. Null pointer are not allowed and checked with an assert.
    pub fn from_ptr(ptr:*mut {1:}) -> {0:} {{
        {0:} {{ ptr }}
    }}
    /// Create the equivalent {0:}Const version of this object.
    pub fn as_const(&mut self) -> {0:}Const {{
      {0:}Const {{ ptr: self.ptr }}
    }}
    /// Generate a {0:}Const pointing to null.
    pub fn is_empty(&mut self) -> bool {{
        self.ptr.is_null()
    }}
"#,
            ti.name, ti.bt_name
        )?;
    }

    for fi in &ti.functions {
        if do_const && !fi.is_const_self {
            continue;
        }
        let mut arg_str = if fi.is_const_self {
            "&self".to_string()
        } else if fi.is_create {
            "".to_string()
        } else {
            "&mut self".to_string()
        };
        let mut param_str = if fi.is_create {
            "".to_string()
        } else {
            "self.ptr".to_string()
        };
        let mut unsafe_str="";
        for (idx, arg) in fi.args.iter().enumerate() {
            if idx != 0 {
                arg_str += &format!(", {}: {}", arg.name, arg.new_type);
                param_str += &format!(", {}{}{}", arg.pre, arg.name, arg.post);
            } else if fi.is_create {
                arg_str += &format!("{}: {}", arg.name, arg.new_type);
                param_str += &format!("{}{}{}", arg.pre, arg.name, arg.post);
            } else {
                arg_str += &format!(", {}: {}", arg.name, arg.new_type);
                param_str += &format!(", {}{}{}", arg.pre, arg.name, arg.post);
            }
            if arg.full_type.contains("*") || arg.full_type.contains("bt_uuid"){
                unsafe_str="unsafe "
            }
        }
        let return_arr = if fi.new_return.is_empty() {
            "".to_string()
        } else {
            format!("-> {}", fi.new_return)
        };
        let null_check=if fi.is_create {""} else {"\n    debug_assert!(!self.ptr.is_null());"};
        write!(
            writer,
            r#"
  #[doc(alias = "{2:}")]
  ///Calls {2:}
  pub {unsafe_str}fn {}({arg_str}) {return_arr} {{{null_check}
    unsafe {{ {}{}({param_str}){} }}
  }}
"#,
            fi.name, fi.pre_fn_call, fi.bt_name, fi.post_fn_call
        )?;
    }
    write!(writer, "\n}}")?;
    Ok(true)
}

fn rewrite_types(result: &mut BTreeMap<String, TypeInfo>, enum_types: &BTreeMap<String, EnumInfo>) {
    lazy_static! {
        static ref DOUBLE_REF: Regex = Regex::new(r"\*.*\*").expect("Failed to compile regex");
    }
    let old_types = result.clone();
    for ti in result.values_mut() {
        for fi in &mut ti.functions {
            if let Some(ret) = &mut fi.bt_return {
                fi.new_return = ret.to_string();
                let base_type = ret.split_whitespace().last().unwrap();
                if ret == "*const ::std::os::raw::c_char" || ret == "*mut ::std::os::raw::c_char" {
                    fi.new_return = "&CStr".to_string();
                    fi.pre_fn_call = "CStr::from_ptr(".to_string();
                    fi.post_fn_call = ")".to_string();
                } else if ret == "bt_bool" {
                    fi.new_return = "bool".to_string();
                    fi.pre_fn_call = "(".to_string();
                    fi.post_fn_call = " as u32) != BT_FALSE".to_string();
                } else if enum_types.contains_key(base_type) {
                    let enums = enum_types.get(base_type).unwrap();
                    let new_name = to_camel_case(&enums.name);
                    fi.new_return = new_name;
                    fi.pre_fn_call = format!(
                        "num::FromPrimitive::from_{}(",
                        get_enum_primitive_type(enums)
                    );
                    fi.post_fn_call = ").unwrap()".to_string();
                } else if old_types.contains_key(base_type) {
                    fi.is_const_return = ret.contains("const");
                    fi.new_return = old_types.get(base_type).unwrap().name.to_string();
                    if fi.is_const_return {
                        fi.new_return += "Const";
                    }
                    fi.pre_fn_call = format!("{}{{ ptr:", fi.new_return);
                    fi.post_fn_call = "}".to_string();
                }
            }
            for arg in &mut fi.args {
                arg.new_type = arg.full_type.to_string();
                if arg.full_type == "*const ::std::os::raw::c_char" {
                    arg.new_type = "&str".to_string();
                    arg.post = ".as_ptr()".to_string();
                    continue;
                }
                if arg.full_type == "*const ::std::os::raw::c_char" {
                    arg.new_type = "&mut str".to_string();
                    arg.post = ".as_mut_ptr()".to_string();
                    continue;
                }
                if arg.full_type == "bt_bool" {
                    arg.new_type = "bool".to_string();
                    arg.pre = "if ".to_string();
                    arg.post = " { BT_TRUE as bt_bool } else { BT_FALSE as bt_bool }".to_string();
                    continue;
                }
                let base_type = arg.full_type.split_whitespace().last().unwrap();
                if enum_types.contains_key(base_type) && !arg.full_type.contains('*') {
                    let enums = enum_types.get(base_type).unwrap();
                    let new_name = to_camel_case(&enums.name);
                    arg.pre = "num::ToPrimitive::to_u32(&".to_string();
                    arg.post = ").unwrap()".to_string();
                    arg.new_type = new_name;
                    continue;
                }
                if old_types.contains_key(base_type) && !DOUBLE_REF.is_match(&arg.full_type) {
                    arg.new_type = format!("&{}", old_types.get(base_type).unwrap().name);
                    if arg.full_type.contains("const") {
                        arg.new_type += "Const";
                    }
                    arg.post = ".ptr".to_string();
                    continue;
                }
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
struct ArgInfo {
    name: String,
    full_type: String,
    new_type: String,
    pre: String,
    post: String,
}

#[derive(Debug, Default, Clone)]
struct EnumValue {
    name: String,
    value: String,
}

#[derive(Debug, Default, Clone)]
struct EnumInfo {
    name: String,
    rust_type: String,
    values: Vec<EnumValue>,
}
impl EnumInfo {
    pub fn add_value(&mut self, arg: EnumValue) {
        self.values.push(arg);
    }
}

#[derive(Debug, Default, Clone)]
struct FuncInfo {
    name: String,
    bt_name: String,
    bt_return: Option<String>,
    args: Vec<ArgInfo>,
    new_return: String,
    pre_fn_call: String,
    post_fn_call: String,
    is_const_self: bool,
    is_const_return: bool,
    is_create: bool,
}
impl FuncInfo {
    pub fn add_argument(&mut self, arg: ArgInfo) {
        self.args.push(arg);
    }
}

#[derive(Debug, Clone, Default)]
struct TypeInfo {
    name: String,
    bt_name: String,
    functions: Vec<FuncInfo>,
}
impl TypeInfo {
    pub fn add_function(&mut self, func: FuncInfo) {
        self.functions.push(func);
    }
}

pub fn to_camel_case(arg: &str) -> String {
    if arg.is_empty() || arg == "_" {
        return "".to_string();
    }
    arg.split('_')
        .map(|x| {
            if x.len()>1 {
                let (first_char, rest) = x.split_at(1);
                format!("{}{}", first_char.to_uppercase(), rest.to_lowercase())
            } else {
                x.to_uppercase()
            }
        })
        .fold("".to_string(), |acc, x| acc + &x)
}

fn handle_function_regex_match(
    cap: regex::Captures,
    types: &mut BTreeMap<String, TypeInfo>,
    enum_types: &BTreeMap<String, EnumInfo>,
) -> bool {
    let fn_name = clean_up_str(cap.name("fn_name").expect("Name is not optional").as_str());
    let fn_self_arg = clean_up_str(
        cap.name("first_arg")
            .expect("Expect the first argument to exist")
            .as_str(),
    );
    let fn_remain_arg = clean_up_str(cap.name("rem_arg").map(|arg| arg.as_str()).unwrap_or(""));
    let fn_return = clean_up_str(cap.name("return").map(|arg| arg.as_str()).unwrap_or(""));
    let self_type = fn_self_arg.split_whitespace().nth_back(0);
    let const_self_type = fn_self_arg.contains("const ");
    if let Some(self_type) = self_type {
        if fn_name.starts_with(self_type) && self_type.starts_with("bt") {
            let ti = get_or_create_typeinfo(types, self_type);
            let mut fi = create_func_info(fn_name, self_type, &fn_return, const_self_type);
            parse_and_add_arguments(fn_remain_arg, &mut fi);
            ti.add_function(fi);
            return true;
        }
    }
    if !fn_return.is_empty() {
        let mut base_type = fn_return.split_whitespace().last().unwrap().to_string();
        if fn_name.contains("create")
            && base_type.starts_with("bt")
        {
            if enum_types.contains_key(&base_type) {
                base_type = fn_remain_arg.split_whitespace().last().unwrap().replace(",", "");
                println!("Using {base_type} for function {fn_name} instead of return type");
            }
            let ti = get_or_create_typeinfo(types, &base_type);
            if fn_name.starts_with(&base_type) {
                let mut fi = create_func_info(fn_name, &base_type, &fn_return, false);
                fi.is_create = true;
                parse_and_add_arguments(format!("{}{}", &fn_self_arg, &fn_remain_arg), &mut fi);
                ti.add_function(fi);
                return true;
            }
        }
    }
    println!("Unhandled function {}", cap.get(0).unwrap().as_str());
    false
}

fn create_func_info(fn_name: String, self_type: &str, fn_return: &str, const_self_type:bool) -> FuncInfo {
    let rust_fn_name = &fn_name[self_type.len() + 1..];
    let bt_return = if fn_return.is_empty() {
        None
    } else {
        assert!(fn_return.starts_with("-> "));
        Some(fn_return[2..].trim().to_string())
    };
    FuncInfo {
        name: rust_fn_name.to_string(),
        bt_name: fn_name,
        bt_return,
        is_const_self: const_self_type,
        ..Default::default()
    }
}

fn parse_and_add_arguments(fn_remain_arg: String, fi: &mut FuncInfo) {
    fn_remain_arg
        .split(',')
        .map(|a| {
            let split = a.trim().split_once(':');
            if let Some((var_name, type_arg)) = split {
                Some(ArgInfo {
                    name: var_name.trim().to_string(),
                    full_type: type_arg.trim().to_string(),
                    ..Default::default()
                })
            } else {
                None
            }
        })
        .for_each(|x| {
            if let Some(x) = x {
                fi.add_argument(x)
            }
        });
}

fn get_or_create_typeinfo<'a>(
    types: &'a mut BTreeMap<String, TypeInfo>,
    self_type: &'a str,
) -> &'a mut TypeInfo {
    let ti = if !types.contains_key(self_type) {
        let new_type = TypeInfo {
            name: to_camel_case(self_type),
            bt_name: self_type.to_string(),
            functions: Vec::new(),
        };
        types.insert(self_type.to_string(), new_type);
        types.get_mut(self_type).unwrap()
    } else {
        types.get_mut(self_type).unwrap()
    };
    ti
}

fn clean_up_str(str: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"\s+"#).unwrap();
    }
    let clean = str.trim().replace('\n', "");
    RE.replace_all(&clean, " ").to_string()
}
