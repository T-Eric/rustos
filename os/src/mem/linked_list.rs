// CS104e上的极简链表，只存储地址

use core::ptr;

/// 这个链表的指针直接指向下一个指针值的地址，相当于什么也不挂载的空心灯笼
#[derive(Debug, Copy, Clone)]
pub struct LinkedList {
    head: *mut usize,
}

unsafe impl Send for LinkedList {}

#[allow(dead_code)]
impl LinkedList {
    pub const fn new() -> LinkedList {
        LinkedList {
            head: ptr::null_mut(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }

    pub unsafe fn push(&mut self, entry: *mut usize) {
        *entry = self.head as usize;
        self.head = entry;
    }

    pub unsafe fn pop(&mut self) -> Option<*mut usize> {
        match self.is_empty() {
            true => None,
            false => {
                let ret = self.head;
                self.head = *ret as *mut usize;
                Some(ret)
            }
        }
    }

    pub fn iter(&self) -> Iter {
        Iter {
            cur: self.head,
            list: self,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut {
        IterMut {
            // pre是指向cur的指针，应当被初始化为self.head的引用
            // 第一个as *mut *mut usize强制将创造出的引用转化为指针
            // 第二个是为了保证与pre类型匹配
            pre: &mut self.head as *mut *mut usize as *mut usize,
            cur: self.head,
            list: self,
        }
    }
}
#[allow(dead_code)]
pub struct Iter<'a> {
    cur: *mut usize,
    list: &'a LinkedList,
}

impl<'a> Iterator for Iter<'a> {
    type Item = *mut usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.is_null() {
            None
        } else {
            let item = self.cur;
            self.cur = unsafe { *item as *mut usize };
            Some(item)
        }
    }
}

// 使用非侵入式链表节点，因为IterMut可以涉及插入和删除
// 单向链表记录两个节点即可
pub struct ListNode {
    pre: *mut usize,
    cur: *mut usize,
}

impl ListNode {
    // remove itself
    pub fn remove(self) -> *mut usize {
        unsafe {
            *self.pre = *self.cur;
        }
        self.cur
    }

    pub fn value(&self) -> *mut usize {
        self.cur
    }
}

#[allow(dead_code)]
pub struct IterMut<'a> {
    list: &'a mut LinkedList,
    pre: *mut usize,
    cur: *mut usize,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = ListNode;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.is_null() {
            None
        } else {
            let ret = ListNode {
                pre: self.pre,
                cur: self.cur,
            };
            self.pre = self.cur;
            self.cur = unsafe { *self.cur as *mut usize };
            Some(ret)
        }
    }
}
