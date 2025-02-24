use core::cell::{RefCell, RefMut};

// 包装，然后
pub struct UpSafeCell<T>{
  cell: RefCell<T>,
}

// 挂一个
unsafe impl<T> Sync for UpSafeCell<T>{}

impl <T> UpSafeCell<T>{
  pub unsafe fn new(value:T)->Self{
    Self{cell:RefCell::new(value)}
  }

  pub fn exclusive_access(&self)->RefMut<'_,T>{
    self.cell.borrow_mut()
  }
}