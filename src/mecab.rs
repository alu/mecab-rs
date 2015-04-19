#![allow(non_camel_case_types)]

use std::ffi::{CStr, CString};
use std::str;
use libc::*;

pub const MECAB_NOR_NODE          : i32 = 0;
pub const MECAB_UNK_NODE          : i32 = 1;
pub const MECAB_BOS_NODE          : i32 = 2;
pub const MECAB_EOS_NODE          : i32 = 3;
pub const MECAB_EON_NODE          : i32 = 4;

pub const MECAB_SYS_DIC           : i32 = 0;
pub const MECAB_USR_DIC           : i32 = 1;
pub const MECAB_UNK_DIC           : i32 = 2;

pub const MECAB_ONE_BEST          : i32 = 1;
pub const MECAB_NBEST             : i32 = 2;
pub const MECAB_PARTIAL           : i32 = 4;
pub const MECAB_MARGINAL_PROB     : i32 = 8;
pub const MECAB_ALTERNATIVE       : i32 = 16;
pub const MECAB_ALL_MORPH         : i32 = 32;
pub const MECAB_ALLOCATE_SENTENCE : i32 = 64;

pub const MECAB_ANY_BOUNDARY      : i32 = 0;
pub const MECAB_TOKEN_BOUNDARY    : i32 = 1;
pub const MECAB_INSIDE_TOKEN      : i32 = 2;

#[link(name="mecab")]
extern {
  fn mecab_new2(arg: *const c_char) -> *mut c_void;
  fn mecab_version() -> *const c_char;
  fn mecab_strerror(mecab: *mut c_void) -> *const c_char;
  fn mecab_destroy(mecab: *mut c_void);
  fn mecab_get_partial(mecab: *mut c_void) -> c_int;
  fn mecab_set_partial(mecab: *mut c_void, partial: c_int);
  fn mecab_get_theta(mecab: *mut c_void) -> c_float;
  fn mecab_set_theta(mecab: *mut c_void, theta: c_float);
  fn mecab_get_lattice_level(mecab: *mut c_void) -> c_int;
  fn mecab_set_lattice_level(mecab: *mut c_void, level: c_int);
  fn mecab_get_all_morphs(mecab: *mut c_void) -> c_int;
  fn mecab_set_all_morphs(mecab: *mut c_void, all_morphs: c_int);
  fn mecab_parse_lattice(mecab: *mut c_void, lattice: *mut c_void) -> c_int;
  fn mecab_sparse_tostr(mecab: *mut c_void, str: *const c_char) -> *const c_char;
  fn mecab_sparse_tonode(mecab: *mut c_void, str: *const c_char) -> *const node_t;
  fn mecab_nbest_sparse_tostr(mecab: *mut c_void, N: size_t, str: *const c_char) -> *const c_char;
  fn mecab_nbest_init(mecab: *mut c_void, str: *const c_char) -> c_int;
  fn mecab_nbest_next_tostr(mecab: *mut c_void) -> *const c_char;
  fn mecab_nbest_next_tonode(mecab: *mut c_void) -> *const node_t;
  fn mecab_format_node(mecab: *mut c_void, node: *const node_t) -> *const c_char;
  fn mecab_dictionary_info(mecab: *mut c_void) -> *const dictionary_info_t;

  fn mecab_lattice_new() -> *mut c_void;
  fn mecab_lattice_destroy(lattice: *mut c_void);
  fn mecab_lattice_clear(lattice: *mut c_void);
  fn mecab_lattice_is_available(lattice: *mut c_void) -> c_int;
  fn mecab_lattice_get_bos_node(lattice: *mut c_void) -> *mut node_t;
  fn mecab_lattice_get_eos_node(lattice: *mut c_void) -> *mut node_t;
  fn mecab_lattice_get_begin_nodes(lattice: *mut c_void, pos: size_t) -> *const node_t;
  fn mecab_lattice_get_end_nodes(lattice: *mut c_void, pos: size_t) -> *const node_t;
  fn mecab_lattice_get_sentence(lattice: *mut c_void) -> *const c_char;
  fn mecab_lattice_set_sentence(lattice: *mut c_void, sentence: *const c_char);
  fn mecab_lattice_get_size(lattice: *mut c_void) -> size_t;
  fn mecab_lattice_get_z(lattice: *mut c_void) -> c_double;
  fn mecab_lattice_set_z(lattice: *mut c_void, Z: c_double);
  fn mecab_lattice_get_theta(lattice: *mut c_void) -> c_double;
  fn mecab_lattice_set_theta(lattice: *mut c_void, theta: c_double);
  fn mecab_lattice_next(lattice: *mut c_void) -> c_int;
  fn mecab_lattice_get_request_type(lattice: *mut c_void) -> c_int;
  fn mecab_lattice_has_request_type(lattice: *mut c_void, request_type: c_int) -> c_int;
  fn mecab_lattice_set_request_type(lattice: *mut c_void, request_type: c_int);
  fn mecab_lattice_add_request_type(lattice: *mut c_void, request_type: c_int);
  fn mecab_lattice_remove_request_type(lattice: *mut c_void, request_type: c_int);
  fn mecab_lattice_tostr(lattice: *mut c_void) -> *const c_char;
  fn mecab_lattice_nbest_tostr(lattice: *mut c_void, N: i64) -> *const c_char;
  fn mecab_lattice_has_constraint(lattice: *mut c_void) -> c_int;
  fn mecab_lattice_get_boundary_constraint(lattice: *mut c_void, pos: u64) -> c_int;
  fn mecab_lattice_get_feature_constraint(lattice: *mut c_void, pos: u64) -> *const c_char;
  fn mecab_lattice_set_boundary_constraint(lattice: *mut c_void, pos: u64, boundary_type: i32);
  fn mecab_lattice_set_feature_constraint(lattice: *mut c_void, begin_pos: u64, end_pos: u64, feature: *const c_char);
  fn mecab_lattice_set_result(lattice: *mut c_void, result: *const c_char);
  fn mecab_lattice_strerror(lattice: *mut c_void) -> *const c_char;

  fn mecab_model_new2(arg: *const c_char) -> *mut c_void;
  fn mecab_model_destroy(model: *mut c_void);
  fn mecab_model_new_tagger(model: *mut c_void) -> *mut c_void;
  fn mecab_model_new_lattice(model: *mut c_void) -> *mut c_void;
  fn mecab_model_swap(model: *mut c_void, new_model: *mut c_void) -> c_int;
  fn mecab_model_dictionary_info(model: *mut c_void) -> *const dictionary_info_t;
  fn mecab_model_transition_cost(model: *mut c_void, rcAttr: c_ushort, lcAttr: c_ushort) -> c_int;
  fn mecab_model_lookup(model: *mut c_void, begin: *const c_char, end: *const c_char, lattice: *mut c_void) -> *const node_t;
}

pub fn version() -> String {
  unsafe {
    ptr_to_string(mecab_version())
  }
}

pub struct Tagger {
  inner: *mut c_void
}

impl Tagger {
  pub fn new(arg: &str) -> Tagger {
    unsafe {
      Tagger {
        inner: mecab_new2(str_to_ptr(arg))
      }
    }
  }

  pub fn get_last_error(&self) -> String {
    unsafe {
      ptr_to_string(mecab_strerror(self.inner))
    }
  }

  pub fn partial(&self) -> i32 {
    unsafe {
      mecab_get_partial(self.inner)
    }
  }

  pub fn set_partial(&self, partial: i32) {
    unsafe {
      mecab_set_partial(self.inner, partial);
    }
  }

  pub fn theta(&self) -> f32 {
    unsafe {
      mecab_get_theta(self.inner)
    }
  }

  pub fn set_theata(&self, theta: f32) {
    unsafe {
      mecab_set_theta(self.inner, theta);
    }
  }

  pub fn lattice_level(&self) -> i32 {
    unsafe {
      mecab_get_lattice_level(self.inner)
    }
  }

  pub fn set_lattice_level(&self, level: i32) {
    unsafe {
      mecab_set_lattice_level(self.inner, level);
    }
  }

  pub fn all_morphs(&self) -> i32 {
    unsafe {
      mecab_get_all_morphs(self.inner)
    }
  }

  pub fn set_all_morphs(&self, all_morphs: i32) {
    unsafe {
      mecab_set_all_morphs(self.inner, all_morphs);
    }
  }

  pub fn parse(&self, latice: &Lattice) -> i32 {
    unsafe {
      mecab_parse_lattice(self.inner, latice.inner)
    }
  }

  pub fn parse_str(&self, input: &str) -> String {
    unsafe {
      ptr_to_string(mecab_sparse_tostr(self.inner, str_to_ptr(input)))
    }
  }

  pub fn parse_to_node(&self, input: &str) -> Node {
    unsafe {
      Node::new(mecab_sparse_tonode(self.inner, str_to_ptr(input)))
    }
  }

  pub fn parse_nbest(&self, n: u64, input: &str) -> String {
    unsafe {
      ptr_to_string(mecab_nbest_sparse_tostr(self.inner, n, str_to_ptr(input)))
    }
  }

  pub fn parse_nbest_init(&self, input: &str) -> i32 {
    unsafe {
      mecab_nbest_init(self.inner, str_to_ptr(input))
    }
  }

  pub fn next(&self) -> Option<String> {
    unsafe {
      let ptr = mecab_nbest_next_tostr(self.inner);
      if !ptr.is_null() {
        Some(ptr_to_string(ptr))
      } else {
        None
      }
    }
  }

  pub fn next_node(&self) -> Option<Node> {
    unsafe {
      let ptr = mecab_nbest_next_tonode(self.inner);
      if !ptr.is_null() {
        Some(Node::new(ptr))
      } else {
        None
      }
    }
  }

  pub fn format_node(&self, node: Node) -> String {
    unsafe {
      ptr_to_string(mecab_format_node(self.inner, node.inner))
    }
  }

  
  pub fn dictionary_info(&self) -> DictionaryInfo {
    unsafe {
      DictionaryInfo::new(mecab_dictionary_info(self.inner))
    }
  }
}

impl Drop for Tagger {
  fn drop(&mut self) {
    unsafe {
      mecab_destroy(self.inner);
    }
  }
}

pub struct Lattice {
  inner: *mut c_void
}

impl Lattice {
  pub fn new() -> Lattice {
    unsafe {
      Lattice {
        inner: mecab_lattice_new()
      }
    }
  }

  pub fn clear(&self) {
    unsafe {
      mecab_lattice_clear(self.inner);
    }
  }

  pub fn is_available(&self) -> bool {
    unsafe {
      mecab_lattice_is_available(self.inner) != 0
    }
  }

  pub fn bos_node(&self) -> Node {
    unsafe {
      Node::new(mecab_lattice_get_bos_node(self.inner))
    }
  }

  pub fn eos_node(&self) -> Node {
    unsafe {
      Node::new(mecab_lattice_get_eos_node(self.inner))
    }
  }

  pub fn begin_nodes(&self, pos: u64) -> Option<Node> {
    unsafe {
      let raw_node = mecab_lattice_get_begin_nodes(self.inner, pos);
      if !raw_node.is_null() {
        Some(Node::new(raw_node))
      } else {
        None
      }
    }
  }

  pub fn end_nodes(&self, pos: u64) -> Option<Node> {
    unsafe {
      let raw_node = mecab_lattice_get_end_nodes(self.inner, pos);
      if !raw_node.is_null() {
        Some(Node::new(raw_node))
      } else {
        None
      }
    }
  }

  pub fn sentence(&self) -> String {
    unsafe {
      ptr_to_string(mecab_lattice_get_sentence(self.inner))
    }
  }

  pub fn set_sentence(&self, sentence: &str) {
    unsafe {
      mecab_lattice_set_sentence(self.inner, str_to_ptr(sentence));
    }
  }

  pub fn size(&self) -> u64 {
    unsafe {
      mecab_lattice_get_size(self.inner)
    }
  }

  pub fn z(&self) -> f64 {
    unsafe {
      mecab_lattice_get_z(self.inner)
    }
  }

  pub fn set_z(&self, z: f64) {
    unsafe {
      mecab_lattice_set_z(self.inner, z);
    }
  }

  pub fn theta(&self) -> f64 {
    unsafe {
      mecab_lattice_get_theta(self.inner)
    }
  }

  pub fn set_theta(&self, theta: f64) {
    unsafe {
      mecab_lattice_set_theta(self.inner, theta);
    }
  }

  pub fn next(&self) -> i32 {
    unsafe {
      mecab_lattice_next(self.inner)
    }
  }

  pub fn request_type(&self) -> i32 {
    unsafe {
      mecab_lattice_get_request_type(self.inner)
    }
  }

  pub fn has_request_type(&self, request_type: i32) -> bool {
    unsafe {
      mecab_lattice_has_request_type(self.inner, request_type) != 0
    }
  }

  pub fn set_request_type(&self, request_type: i32) {
    unsafe {
      mecab_lattice_set_request_type(self.inner, request_type);
    }
  }

  pub fn add_request_type(&self, request_type: i32) {
    unsafe {
      mecab_lattice_add_request_type(self.inner, request_type);
    }
  }

  pub fn remove_request_type(&self, request_type: i32) {
    unsafe {
      mecab_lattice_remove_request_type(self.inner, request_type);
    }
  }

  pub fn to_string(&self) -> String {
    unsafe {
      ptr_to_string(mecab_lattice_tostr(self.inner))
    }
  }

  pub fn enum_nbest_as_string(&self, n: i64) -> String {
    unsafe {
      ptr_to_string(mecab_lattice_nbest_tostr(self.inner, n))
    }
  }

  pub fn has_constraint(&self) -> i32 {
    unsafe {
      mecab_lattice_has_constraint(self.inner)
    }
  }

  pub fn boundary_constraint(&self, pos: u64) -> i32 {
    unsafe {
      mecab_lattice_get_boundary_constraint(self.inner, pos)
    }
  }

  pub fn feature_constraint(&self, pos: u64) -> String {
    unsafe {
      ptr_to_string(mecab_lattice_get_feature_constraint(self.inner, pos))
    }
  }

  pub fn set_boundary_constraint(&self, pos: u64, boundary_type: i32) {
    unsafe {
      mecab_lattice_set_boundary_constraint(self.inner, pos, boundary_type);
    }
  }

  pub fn set_feature_constraint(&self, begin_pos: u64, end_pos: u64, feature: &str) {
    unsafe {
      mecab_lattice_set_feature_constraint(self.inner, begin_pos, end_pos, str_to_ptr(feature));
    }
  }

  pub fn set_result(&self, result: &str) {
    unsafe {
      mecab_lattice_set_result(self.inner, str_to_ptr(result))
    }
  }

  pub fn what(&self) -> String {
    unsafe {
      ptr_to_string(mecab_lattice_strerror(self.inner))
    }
  }
}

impl Drop for Lattice {
  fn drop(&mut self) {
    unsafe {
      mecab_lattice_destroy(self.inner);
    }
  }
}

pub struct Model {
  inner: *mut c_void
}

impl Model {
  pub fn new(args: &str) -> Model {
    unsafe {
      Model {
        inner: mecab_model_new2(str_to_ptr(args))
      }
    }
  }

  pub fn create_tagger(&self) -> Tagger {
    unsafe {
      Tagger {
        inner: mecab_model_new_tagger(self.inner)
      }
    }
  }

  pub fn create_lattice(&self) -> Lattice {
    unsafe {
      Lattice {
        inner: mecab_model_new_lattice(self.inner)
      }
    }
  }

  pub fn swap(&self, model: &Model) -> i32 {
    unsafe {
      mecab_model_swap(self.inner, model.inner)
    }
  }

  pub fn dictionary_info(&self) -> DictionaryInfo {
    unsafe {
      DictionaryInfo::new(mecab_model_dictionary_info(self.inner))
    }
  }

  pub fn transition_cost(&self, rc_attr: u16, lc_attr: u16) -> i32 {
    unsafe {
      mecab_model_transition_cost(self.inner, rc_attr, lc_attr)
    }
  }

  pub fn lookup(&self, begin: &str, len: u64, lattice: &Lattice) -> Option<Node> {
    unsafe {
      let raw_node = mecab_model_lookup(self.inner, str_to_ptr(begin), str_to_ptr(begin).offset(len as isize), lattice.inner);
      if !raw_node.is_null() {
        Some(Node::new(raw_node))
      } else {
        None
      }
    }
  }
}

impl Drop for Model {
  fn drop(&mut self) {
    unsafe {
      mecab_model_destroy(self.inner);
    }
  }
}

#[repr(C)]
struct node_t {
  prev: *mut node_t,
  next: *mut node_t,
  enext: *mut node_t,
  bnext: *mut node_t,
  rpath: *mut c_void,
  lpath: *mut c_void,
  surface: *const c_char,
  feature: *const c_char,
  id: c_uint,
  length: c_uint,
  rlength: c_ushort,
  rcattr: c_ushort,
  lcattr: c_ushort,
  posid: c_ushort,
  char_type: c_uchar,
  stat: c_uchar,
  isbest: c_uchar,
  alpha: c_float,
  beta: c_float,
  prob: c_float,
  wcost: c_short,
  cost: c_long
}

pub struct Node {
  inner: *const node_t,
  prev: *mut node_t,
  next: *mut node_t,
  enext: *mut node_t,
  bnext: *mut node_t,

  pub surface: String,
  pub feature: String,
  pub id: u32,
  pub length: u32,
  pub rlength: u16,
  pub rcattr: u16,
  pub lcattr: u16,
  pub posid: u16,
  pub char_type: u8,
  pub stat: u8,
  pub isbest: bool,
  pub alpha: f32,
  pub beta: f32,
  pub prob: f32,
  pub wcost: i16,
  pub cost: i64
}

impl Node {
  fn new(raw_ptr: *const node_t) -> Node {
    unsafe {
      let ref raw_node = *raw_ptr;

      Node {
        inner: raw_ptr,
        prev: raw_node.prev,
        next: raw_node.next,
        enext: raw_node.enext,
        bnext: raw_node.bnext,
        surface: ptr_to_string(raw_node.surface),
        feature: ptr_to_string(raw_node.feature),
        id: raw_node.id,
        length: raw_node.length,
        rlength: raw_node.rlength,
        rcattr: raw_node.rcattr,
        lcattr: raw_node.lcattr,
        posid: raw_node.posid,
        char_type: raw_node.char_type,
        stat: raw_node.stat,
        isbest: raw_node.isbest != 0,
        alpha: raw_node.alpha,
        beta: raw_node.beta,
        prob: raw_node.prob,
        wcost: raw_node.wcost,
        cost: raw_node.cost
      }
    }
  }

  pub fn prev(&self) -> Option<Node> {
    if !self.prev.is_null() {
      Some(Node::new(self.prev))
    } else {
      None
    }
  }

  pub fn next(&self) -> Option<Node> {
    if !self.next.is_null() {
      Some(Node::new(self.next))
    } else {
      None
    }
  }

  pub fn enext(&self) -> Option<Node> {
    if !self.enext.is_null() {
      Some(Node::new(self.enext))
    } else {
      None
    }
  }

  pub fn bnext(&self) -> Option<Node> {
    if !self.bnext.is_null() {
      Some(Node::new(self.bnext))
    } else {
      None
    }
  }
}

#[repr(C)]
struct dictionary_info_t {
  filename: *const c_char,
  charset: *const c_char,
  size: c_uint,
  dict_type: c_int,
  lsize: c_uint,
  rsize: c_uint,
  version: c_ushort,
  next: *mut dictionary_info_t
}

pub struct DictionaryInfo {
  pub filename: String,
  pub charset: String,
  pub size: u32,
  pub dict_type: i32,
  pub lsize: u32,
  pub rsize: u32,
  pub version: u16,
  next: *mut dictionary_info_t,
}

impl DictionaryInfo {
  fn new(raw_ptr: *const dictionary_info_t) -> DictionaryInfo {
    unsafe {
      let ref dict = *raw_ptr;

      DictionaryInfo {
        next: dict.next,
        filename: ptr_to_string(dict.filename),
        charset: ptr_to_string(dict.charset),
        size: dict.size,
        dict_type: dict.dict_type,
        lsize: dict.lsize,
        rsize: dict.rsize,
        version: dict.version,
      }
    }
  }

  pub fn next(&self) -> Option<DictionaryInfo> {
    if !self.next.is_null() {
      Some(DictionaryInfo::new(self.next))
    } else {
      None
    }
  }
}

fn str_to_ptr(input: &str) -> *const i8 {
  CString::new(input.as_bytes()).unwrap().as_bytes_with_nul().as_ptr() as *const i8
}

fn ptr_to_string(ptr: *const c_char) -> String {
  unsafe {
    str::from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()).to_string()
  }
}