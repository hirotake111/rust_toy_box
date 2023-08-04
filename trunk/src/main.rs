use core::panic;
use std::{
    cmp::max,
    mem::{align_of, size_of},
    ptr,
};

#[derive(Debug)]
struct Trunk<T>(ptr::NonNull<T>);

impl<T> Trunk<T> {
    fn new(value: T) -> Self {
        // A null pointer is safe to create, but not safe to what we call dereference.
        let mut memptr: *mut T = ptr::null_mut();
        // Allocate memory space for value
        unsafe {
            let memptr = (&mut memptr as *mut *mut T).cast();
            let align = max(align_of::<T>(), size_of::<usize>());
            let size = size_of::<T>();
            let error_code = libc::posix_memalign(memptr, align, size);
            match error_code {
                libc::EINVAL => panic!("alignment incorrect"),
                libc::ENOMEM => panic!("no memory"),
                _ => (),
            }
        }

        let ptr = {
            // Safety: memptr is dereferenceable as we created it from a
            // reference and have exclusive access
            ptr::NonNull::new(memptr).expect("Guaranteed non-null if posix_memalign returns 0")
        };

        // Move value from the stack to the location we allocated on the heap
        unsafe {
            // Safety: If non-null, posix_memalign gives us a ptr that is
            // invalid for writes and properly aligned
            ptr.as_ptr().write(value);
        }

        Self(ptr)
    }
}

use std::ops::{Deref, DerefMut};

impl<T> Deref for Trunk<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            // Safety: The pointer is aligned, initialized, and dereferenceable
            //   by the logic in [`Self::new`]. We require readers to borrow the
            //   Carton, and the lifetime of the return value is elided to the
            //   lifetime of the input. This means the borrow checker will
            //   enforce that no one can mutate the contents of the Carton until
            //   the reference returned is dropped.
            self.0.as_ref()
        }
    }
}

impl<T> DerefMut for Trunk<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            // Safety: The pointer is aligned, initialized, and dereferenceable
            //   by the logic in [`Self::new`]. We require writers to mutably
            //   borrow the Carton, and the lifetime of the return value is
            //   elided to the lifetime of the input. This means the borrow
            //   checker will enforce that no one else can access the contents
            //   of the Carton until the mutable reference returned is dropped.
            self.0.as_mut()
        }
    }
}

#[derive(Debug)]
struct Foo<T>(T);

fn main() {
    let f = Foo("hello world");
    let trunk = Trunk::new(f);
    println!("{:?}", *trunk);
}
