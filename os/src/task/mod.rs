mod context;
mod switch;
mod task_info;

use crate::config::MAX_APP_NUM;
use crate::loader::{get_num_app, init_app_cx};
use crate::sync::UpSafeCell;
use crate::tesbi::sbi::shutdown;
use crate::Log;
use context::TaskContext;
use lazy_static::lazy_static;
use switch::__switch;
use task_info::TaskControlBlock;
use task_info::TaskStatus;

pub struct TaskManager {
    num_app: usize,
    inner: UpSafeCell<TaskManagerInner>,
}

// 所有任务的dashboard
struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
}

// 与APP_MANAGER类似，TASK_MANAGER也是一个全局变量，用于管理所有的任务
lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let mut tasks = [TaskControlBlock {
            task_cx: TaskContext::zero_init(),
            task_status: TaskStatus::UnInit,
        }; MAX_APP_NUM];
        for i in 0..num_app {
            tasks[i].task_cx = TaskContext::goto_restore(init_app_cx(i));
            tasks[i].task_status = TaskStatus::Ready;
        }
        TaskManager {
            num_app,
            inner: unsafe {
                UpSafeCell::new(TaskManagerInner {
                    tasks,
                    current_task: 0,
                })
            },
        }
    };
}

impl TaskManager {
    fn mark_cur_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let cur = inner.current_task; // borrow as local immutable then copy
        inner.tasks[cur].task_status = TaskStatus::Exited; // 'cause here requires a mutable borrow
    }

    // 'suspend' means the task is not running, but its context is still in memory
    // so we just need mark it 'ready' so it will be scheduled again in the future
    fn mark_cur_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let cur = inner.current_task;
        inner.tasks[cur].task_status = TaskStatus::Ready;
    }

    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let cur = inner.current_task;
        // traverse to find a Ready
        (cur + 1..cur + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
    }

    fn run_next_task(&self) {
        if let Some(nxt) = self.find_next_task() {
            let mut inner = self.inner.exclusive_access();
            let cur = inner.current_task;
            inner.tasks[nxt].task_status = TaskStatus::Running;
            inner.current_task = nxt;
            let cur_task_cx_ptr = &mut inner.tasks[cur].task_cx as *mut TaskContext;
            let nxt_task_cx_ptr = &inner.tasks[nxt].task_cx as *const TaskContext;
            // inner是可变借用，如果不drop，switch函数会一直占用inner这个可变借用
            // 因为switch函数需要修改inner的current_task
            drop(inner);

            unsafe {
                __switch(cur_task_cx_ptr, nxt_task_cx_ptr);
            }
            // go back to user mode
        } else {
            info_info!("All applications (tasks) completed!");
            shutdown();
        }
    }

    fn run_first_task(&self) -> ! {
        let mut inner = self.inner.exclusive_access();
        let task0 = &mut inner.tasks[0];
        task0.task_status = TaskStatus::Running;
        let nxt_task_cx_ptr = &task0.task_cx as *const TaskContext;
        drop(inner);
        let mut _unused = TaskContext::zero_init();
        unsafe {
            __switch(&mut _unused as *mut TaskContext, nxt_task_cx_ptr);
        }
        panic!("unreachable code in run_first_task!");
    }
}

fn mark_cur_exited() {
    TASK_MANAGER.mark_cur_exited();
}

fn mark_cur_suspended() {
    TASK_MANAGER.mark_cur_suspended();
}

fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

pub fn suspend_cur_and_run_next() {
    mark_cur_suspended();
    run_next_task();
}

pub fn exit_cur_and_run_next() {
    mark_cur_exited();
    run_next_task();
}
