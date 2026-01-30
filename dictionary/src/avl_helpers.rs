use super::dict::*;
use core::ptr::null_mut;
use libc::free;
use std::ptr;

use crate::MyString;

fn height(n: *mut Node) -> i32 {
    unsafe { if n.is_null() { 0 } else { (*n).height } }
}

fn update_height(n: *mut Node) {
    unsafe {
        let hl = height((*n).left);
        let hr = height((*n).right);

        if hl > hr {
            (*n).height = 1 + hl;
        } else {
            (*n).height = 1 + hr;
        }
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

fn rr(p: *mut Node) -> *mut Node {
    unsafe {
        let x = (*p).right;
        let t2 = (*x).left;
        (*p).right = t2;
        (*x).left = p;

        update_height(p);
        update_height(x);

        x
    }
}

fn ll(p: *mut Node) -> *mut Node {
    unsafe {
        let x = (*p).left;
        let t2 = (*x).right;

        (*p).left = t2;
        (*x).right = p;

        update_height(p);
        update_height(x);

        x
    }
}

fn rl(p: *mut Node) -> *mut Node {
    unsafe {
        (*p).right = ll((*p).right);
        rr(p)
    }
}

fn lr(p: *mut Node) -> *mut Node {
    unsafe {
        (*p).left = rr((*p).left);
        ll(p)
    }
}

fn balance(n: *mut Node) -> *mut Node {
    unsafe {
        update_height(n);

        let bf = balance_factor(n);

        if bf > 1 {
            let left_bf = balance_factor((*n).left);
            if left_bf >= 0 {
                return ll(n);
            } else {
                return lr(n);
            }
        }

        if bf < -1 {
            let right_bf = balance_factor((*n).right);
            if right_bf <= 0 {
                return rr(n);
            } else {
                return rl(n);
            }
        }

        n
    }
}

pub fn insert_node(node: *mut Node, key: u64, value: MyString) -> *mut Node {
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

pub fn find_node(mut node: *mut Node, key: u64) -> Option<*mut Node> {
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

pub fn remove_node(node: *mut Node, key: u64) -> *mut Node {
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
                let temp;

                if (*node).left.is_null() {
                    temp = (*node).right
                } else {
                    temp = (*node).left
                };

                ptr::drop_in_place(&mut (*node).value);
                free(node as *mut _);
                return temp;
            }

            let succ = min_node((*node).right);

            let tmp_key = (*node).key;
            (*node).key = (*succ).key;
            (*succ).key = tmp_key;

            let tmp_val = ptr::read(&(*node).value);
            (*node).value = ptr::read(&(*succ).value);
            (*succ).value = tmp_val;
            (*node).right = remove_node((*node).right, (*succ).key);
        }
        balance(node)
    }
}

pub unsafe fn free_tree(node: *mut Node) {
    unsafe {
        if node.is_null() {
            return;
        }

        free_tree((*node).left);
        free_tree((*node).right);

        ptr::drop_in_place(&mut (*node).value);

        (*node).left = null_mut();
        (*node).right = null_mut();

        free(node as *mut _);
    }
}
