extern crate libc;
use core::ptr::null_mut;
use libc::{free, malloc};
use std::ptr;
// ========================= // String // =========================
#[repr(C)]
pub struct SimpleString {
    ptr: *mut u8,
    len: usize,
}
impl SimpleString {
    pub fn new_from_str(s: &str) -> Self {
        unsafe {
            let len = s.len();
            let mem = malloc(len) as *mut u8;
            if mem.is_null() {
                return Self {
                    ptr: null_mut(),
                    len: 0,
                };
            }
            core::ptr::copy_nonoverlapping(s.as_ptr(), mem, len);
            Self { ptr: mem, len }
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn as_str(&self) -> &str {
        unsafe {
            let slice = core::slice::from_raw_parts(self.ptr, self.len);
            core::str::from_utf8_unchecked(slice)
        }
    }
}
impl Drop for SimpleString {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                free(self.ptr as *mut _);
            }
        }
    }
} // ========================= // Dict // ========================= 
struct Node {
    key: u64,
    value: SimpleString,
    height: i32,
    left: *mut Node,
    right: *mut Node,
}
impl Node {
    fn new(key: u64, value: SimpleString) -> *mut Node {
        unsafe {
            let mem = malloc(core::mem::size_of::<Node>()) as *mut Node;
            if mem.is_null() {
                return null_mut();
            }
            let node = Node {
                key,
                value,
                height: 1,
                left: null_mut(),
                right: null_mut(),
            };
            core::ptr::write(mem, node);
            mem
        }
    }
}
pub struct Dictionary {
    root: *mut Node,
}
impl Dictionary {
    pub fn new() -> Self {
        Self { root: null_mut() }
    }
    pub fn insert(&mut self, key: u64, value: SimpleString) {
        self.root = insert_node(self.root, key, value);
    }
    pub fn contains(&self, key: u64) -> bool {
        find_node(self.root, key).is_some()
    }
    pub fn get(&self, key: u64) -> Option<&SimpleString> {
        unsafe {
            match find_node(self.root, key) {
                Some(n) => Some(&(*n).value),
                None => None,
            }
        }
    }
    pub fn remove(&mut self, key: u64) {
        self.root = remove_node(self.root, key);
    }
}
impl Drop for Dictionary {
    fn drop(&mut self) {
        unsafe {
            free_tree(self.root);
        }
        self.root = core::ptr::null_mut();
    }
} // ========================= // Funkcje AVL // ========================= 
fn height(n: *mut Node) -> i32 {
    unsafe { if n.is_null() { 0 } else { (*n).height } }
}
fn update_height(n: *mut Node) {
    unsafe {
        let hl = height((*n).left);
        let hr = height((*n).right);
        (*n).height = 1 + if hl > hr { hl } else { hr };
    }
}
fn balance_factor(n: *mut Node) -> i32 {
    unsafe {
        if n.is_null() {
            0
        } else {
            height((*n).left) - height((*n).right)
        }
    }
}
fn rotate_right(y: *mut Node) -> *mut Node {
    unsafe {
        let x = (*y).left;
        let t2 = (*x).right;
        (*x).right = y;
        (*y).left = t2;
        update_height(y);
        update_height(x);
        x
    }
}
fn rotate_left(x: *mut Node) -> *mut Node {
    unsafe {
        let y = (*x).right;
        let t2 = (*y).left;
        (*y).left = x;
        (*x).right = t2;
        update_height(x);
        update_height(y);
        y
    }
}
fn balance(n: *mut Node) -> *mut Node {
    unsafe {
        update_height(n);
        let bf = balance_factor(n);
        if bf > 1 {
            if balance_factor((*n).left) < 0 {
                (*n).left = rotate_left((*n).left);
            }
            return rotate_right(n);
        }
        if bf < -1 {
            if balance_factor((*n).right) > 0 {
                (*n).right = rotate_right((*n).right);
            }
            return rotate_left(n);
        }
        n
    }
}
fn insert_node(node: *mut Node, key: u64, value: SimpleString) -> *mut Node {
    if node.is_null() {
        return Node::new(key, value);
    }
    unsafe {
        if key < (*node).key {
            (*node).left = insert_node((*node).left, key, value);
        } else if key > (*node).key {
            (*node).right = insert_node((*node).right, key, value);
        }
        balance(node)
    }
}
fn find_node(mut node: *mut Node, key: u64) -> Option<*mut Node> {
    unsafe {
        while !node.is_null() {
            if key == (*node).key {
                return Some(node);
            } else if key < (*node).key {
                node = (*node).left;
            } else {
                node = (*node).right;
            }
        }
        None
    }
}
fn min_node(mut n: *mut Node) -> *mut Node {
    unsafe {
        while !(*n).left.is_null() {
            n = (*n).left;
        }
        n
    }
}
fn remove_node(node: *mut Node, key: u64) -> *mut Node {
    if node.is_null() {
        return null_mut();
    }
    unsafe {
        if key < (*node).key {
            (*node).left = remove_node((*node).left, key);
        } else if key > (*node).key {
            (*node).right = remove_node((*node).right, key);
        } else {
            if (*node).left.is_null() || (*node).right.is_null() {
                let temp = if (*node).left.is_null() {
                    (*node).right
                } else {
                    (*node).left
                };
                ptr::drop_in_place(&mut (*node).value);
                free(node as *mut _);
                return temp;
            }
            let succ = min_node((*node).right);
            core::mem::swap(&mut (*node).key, &mut (*succ).key);
            core::mem::swap(&mut (*node).value, &mut (*succ).value);
            (*node).right = remove_node((*node).right, (*succ).key);
        }
        balance(node)
    }
}
unsafe fn free_tree(node: *mut Node) {
    unsafe {
        if node.is_null() {
            return;
        }
        free_tree((*node).left);
        free_tree((*node).right);
        ptr::drop_in_place(&mut (*node).value);
        (*node).left = null_mut();
        (*node).right = null_mut();
        if !node.is_null() {
            free(node as *mut _);
        }
    }
} // ========================= // Makro // =========================
#[macro_export]
macro_rules! dict {
    ( $( $k:expr => $v:expr ),* $(,)? ) => {{
        let mut d = $crate::Dictionary::new();
        $(
            d.insert($k, $crate::SimpleString::new_from_str($v));
        )*
        d
    }};
}

// ========================= // C API // =========================
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_new() -> *mut Dictionary {
    let d = Dictionary::new();
    Box::into_raw(Box::new(d))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_insert(dict: *mut Dictionary, key: u64, val: *const u8, len: usize) {
    unsafe {
        let slice = core::slice::from_raw_parts(val, len);
        let s = core::str::from_utf8_unchecked(slice);
        (*dict).insert(key, SimpleString::new_from_str(s));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_contains(dict: *mut Dictionary, key: u64) -> bool {
    unsafe { (*dict).contains(key) }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_get(dict: *mut Dictionary, key: u64, len: *mut usize) -> *const u8 {
    unsafe {
        match (*dict).get(key) {
            Some(s) => {
                *len = s.len();
                s.ptr as *const u8
            }
            None => null_mut(),
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_remove(dict: *mut Dictionary, key: u64) {
    unsafe {
        (*dict).remove(key);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_free(dict: *mut Dictionary) {
    unsafe {
        drop(Box::from_raw(dict));
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn string_test1() {
        let s = SimpleString::new_from_str("test");
        assert_eq!(s.as_str(), "test");
    }
    #[test]
    fn string_test2() {
        let s = SimpleString::new_from_str("another test string");
        assert_eq!(s.as_str(), "another test string");
        assert_eq!(s.len(), 19);
        let s = SimpleString::new_from_str("");
        assert_eq!(s.as_str(), "");
    }
    #[test]
    fn node_test() {
        let n = Node::new(42, SimpleString::new_from_str("forty-two"));
        unsafe {
            assert_eq!((*n).key, 42);
            assert_eq!((*n).value.as_str(), "forty-two");
            ptr::drop_in_place(&mut (*n).value);
            free(n as *mut _);
        }
    }
    #[test]
    fn dict_insert_test() {
        let mut d = Dictionary::new();
        d.insert(10, SimpleString::new_from_str("ten"));
    }
    #[test]
    fn dict_get_test() {
        let mut d = Dictionary::new();
        d.insert(20, SimpleString::new_from_str("twenty"));
        let val1 = d.get(20);
        let val2 = d.get(20);
        assert!(val1.is_some());
        assert!(val2.is_some());
        assert_eq!(val1.unwrap().as_str(), "twenty");
        assert_eq!(val2.unwrap().as_str(), "twenty");
    }
    #[test]
    fn dict_remove_test() {
        let mut d = Dictionary::new();
        d.insert(30, SimpleString::new_from_str("thirty"));
        d.remove(30);
        let val = d.get(30);
        assert!(val.is_none());
    }
    #[test]
    fn dict_advanced_test() {
        let mut d = Dictionary::new();
        for i in 1..=100 {
            d.insert(i, SimpleString::new_from_str(&format!("number {}", i)));
        }
        for i in 1..=100 {
            let val = d.get(i);
            assert!(val.is_some());
            assert_eq!(val.unwrap().as_str(), &format!("number {}", i));
        }
        for i in 1..=50 {
            d.remove(i);
        }
        for i in 1..=50 {
            let val = d.get(i);
            assert!(val.is_none());
        }
        for i in 51..=100 {
            let val = d.get(i);
            assert!(val.is_some());
            assert_eq!(val.unwrap().as_str(), &format!("number {}", i));
        }
    }
    #[test]
    fn dict_macro_test() {
        let mut d = dict!(1 => "one", 2 => "two", 3 => "three");
        d.insert(4, SimpleString::new_from_str("four"));
        assert!(d.contains(2));
        let val = d.get(2);
        assert!(val.is_some());
        assert_eq!(val.unwrap().as_str(), "two");
        d.remove(2);
        assert!(!d.contains(2));
        let val = d.get(4);
        assert!(val.is_some());
        assert_eq!(val.unwrap().as_str(), "four");
    }
    #[test]
    fn dict_hello_world_test() {
        let mut d = dict!(1 => "hello", 2 => "world");
        d.insert(3, SimpleString::new_from_str("!"));
        let val = d.contains(2);
        assert!(val);
        d.remove(2);
        let val = d.contains(2);
        assert!(!val);
        let str1 = d.get(1);
        let str3 = d.get(3);
        assert_eq!(str1.unwrap().as_str(), "hello");
        assert_eq!(str3.unwrap().as_str(), "!");
        d.remove(1);
        let None = d.get(1) else {
            panic!("Key 1 should have been removed");
        };
    }
}
