use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;

pub struct C<T> {
    payload: Rc<RefCell<T>>,
}

pub struct R<T> {
    payload: Rc<RefCell<T>>,
}

impl<T> C<T> {
    pub fn new(payload: T) -> Self {
        Self {
            payload: Rc::new(RefCell::new(payload)),
        }
    }

    pub fn r(&self) -> R<T> {
        R {
            payload: Rc::clone(&self.payload),
        }
    }

    pub fn borrow_mut(&mut self) -> RefMut<T> {
        self.payload.borrow_mut()
    }

    pub fn borrow(&self) -> Ref<T> {
        self.payload.borrow()
    }

    pub fn set(&mut self, mut payload: T) {
        std::mem::swap(&mut (*self.payload.borrow_mut()), &mut payload);
    }
}

impl<T> R<T> {
    pub fn borrow(&self) -> Ref<T> {
        self.payload.borrow()
    }
}

impl<T> Clone for R<T> {
    fn clone(&self) -> Self {
        Self {
            payload: Rc::clone(&self.payload),
        }
    }
}
