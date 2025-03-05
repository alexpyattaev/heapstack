#[inline(never)]
pub fn do_work_heap_alloc(alloc: usize, iters: u8) -> u8 {
    let mut buf = Vec::with_capacity(alloc);
    for i in 0..iters {
        for e in buf.iter_mut() {
            *e += i;
        }
    }
    buf.iter().sum()
}
#[inline(never)]
pub fn do_work_stack_alloc_100(iters: u8) -> u8 {
    do_work_stack_alloc::<100>(iters)
}

#[inline(never)]
pub fn do_work_stack_alloc_1000(iters: u8) -> u8 {
    do_work_stack_alloc::<1000>(iters)
}
fn do_work_stack_alloc<const N: usize>(iters: u8) -> u8 {
    let mut buf = [0u8; N];
    for i in 0..iters {
        for e in buf.iter_mut() {
            *e += i;
        }
    }
    buf.iter().sum()
}
