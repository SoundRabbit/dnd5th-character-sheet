use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

thread_local!(static HASHER: RefCell<DefaultHasher> = RefCell::new(DefaultHasher::new()));

pub fn new(x: impl Hash) -> u64 {
    HASHER.with(|hasher| {
        x.hash(&mut (*hasher.borrow_mut()));
        hasher.borrow().finish()
    })
}
