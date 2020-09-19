use std::cell::Cell;

thread_local!(static COUNT: Cell<u64> = Cell::new(0));

pub fn new() -> u64 {
    COUNT.with(|count| {
        let luid = count.get();
        count.set(luid + 1);
        luid
    })
}
