use super::linked_list::LinkedList;
use core::alloc::{GlobalAlloc, Layout};
use core::cmp::{max, min};
use core::ops::Deref;
use spin::Mutex;
#[allow(dead_code)]
pub struct Heap {
    pub layer: [LinkedList; 32],
    total: usize,
    allocated: usize,
}

impl Heap {
    pub const fn new() -> Self {
        Heap {
            layer: [LinkedList::new(); 32],
            total: 0,
            allocated: 0,
        }
    }

    pub unsafe fn manage(&mut self, start: usize, end: usize) {
        assert!(start <= end);
        let mut cur = start;
        let mut total = 0;

        // 每次从剩下的地址中切一块最小的2的幂，把它加入链表
        while cur + size_of::<usize>() <= end {
            let lowbit = cur & (!cur + 1);
            let size = min(lowbit, prev_power_of_2(end - cur));
            self.layer[size.trailing_zeros() as usize].push(cur as *mut usize);
            cur += size;
            total += size;
        }
        self.total += total;
    }

    pub unsafe fn init(&mut self, start: usize, size: usize) {
        self.manage(start, start + size)
    }

    pub unsafe fn alloc(&mut self, layout: Layout) -> Option<*mut u8> {
        let size = max(
            layout.size().next_power_of_two(),
            max(layout.align(), size_of::<usize>()),
        );
        let ord = size.trailing_zeros() as usize; // order in need

        // find a non-empty layer over current layer from down to up
        // the return statements can escape the loop
        for i in ord..self.layer.len() {
            if !self.layer[i].is_empty() {
                //split back
                for j in (ord + 1..=i).rev() {
                    if let Some(block) = self.layer[j].pop() {
                        // push two buddies, firstly bigger one
                        // then when using, the smaller one will come first
                        self.layer[j - 1].push((block as usize + (1 << (j - 1))) as *mut usize);
                        self.layer[j - 1].push(block);
                    } else {
                        return None; // unreachable if properly used
                    }
                }

                // return the one as *void
                let ret = self.layer[ord].pop().expect("Should not be empty!") as *mut u8;
                return if ret.is_null() {
                    None
                } else {
                    self.allocated += size;
                    Some(ret)
                };
            }
        }
        None
    }

    pub unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        assert!(!ptr.is_null());
        let size = max(
            layout.size().next_power_of_two(),
            max(layout.align(), size_of::<usize>()),
        );
        let ord = size.trailing_zeros() as usize;

        // put it back
        self.layer[ord].push(ptr as *mut usize);

        // merge if we can
        let mut cur_ptr = ptr as usize;
        let mut cur_ord = ord;
        while cur_ord < self.layer.len() {
            let buddy = cur_ptr ^ (1 << cur_ord);
            let mut flag = false;
            for block in self.layer[cur_ord].iter_mut() {
                if block.value() as usize == buddy {
                    block.remove();
                    flag = true;
                    break;
                }
            }

            // if there is a buddy, merge them and go up
            // else the upper layer won't have anything to merge
            if flag {
                self.layer[cur_ord].pop();
                cur_ptr = min(cur_ptr, buddy);
                cur_ord += 1;
                self.layer[cur_ord].push(cur_ptr as *mut usize);
            } else {
                break;
            }
        }

        self.allocated -= size;
    }
}

// 取出其中一块2的幂大小的地址
fn prev_power_of_2(num: usize) -> usize {
    1 << (8 * (size_of::<usize>()) - num.leading_zeros() as usize - 1)
}

pub struct LockedHeap(Mutex<Heap>);

impl LockedHeap {
    pub const fn new() -> Self {
        LockedHeap(Mutex::new(Heap::new()))
    }
}

// for direct use
impl Deref for LockedHeap {
    type Target = Mutex<Heap>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

unsafe impl GlobalAlloc for LockedHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0
            .lock()
            .alloc(layout)
            .expect("Heap allocation failed!...")
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.lock().dealloc(ptr, layout)
    }
}
