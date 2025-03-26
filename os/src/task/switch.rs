use super::context::TaskContext;
use core::arch::global_asm;

global_asm!(include_str!("switch.S"));

extern "C" {
    pub fn __switch(cur_task_cx_ptr: *mut TaskContext, nxt_task_cx_ptr: *const TaskContext);
}

/*
上面展示了一个十分标准的外置汇编插入流程：
.S文件写函数，使用extern "C"把它作为rust函数声明
一方面可以把它当rust函数调用，另一方面编译器可以自动完成函数的调用处理
 */
