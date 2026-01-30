use core::ptr::null_mut;

use libc::malloc;

use crate::MyString;
use crate::avl_helpers::{find_node, free_tree, insert_node, remove_node};

pub struct Dictionary {
    pub root: *mut Node,
}

impl Dictionary {
    pub fn new() -> Self {
        Self { root: null_mut() }
    }

    pub fn insert(&mut self, key: u64, value: MyString) {
        self.root = insert_node(self.root, key, value);
    }

    pub fn contains(&self, key: u64) -> bool {
        match find_node(self.root, key) {
            None => false,
            Some(_) => true,
        }
    }

    pub fn get(&self, key: u64) -> Option<&MyString> {
        unsafe { find_node(self.root, key).map(|n| &(*n).value) }
    }

    pub fn remove(&mut self, key: u64) {
        self.root = remove_node(self.root, key);
    }
}

impl Drop for Dictionary {
    fn drop(&mut self) {
        unsafe { free_tree(self.root) }
        self.root = null_mut();
    }
}

pub struct Node {
    pub key: u64,
    pub value: MyString,

    pub height: i32,

    pub left: *mut Node,
    pub right: *mut Node,
}

impl Node {
    pub fn new(key: u64, value: MyString) -> *mut Node {
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
