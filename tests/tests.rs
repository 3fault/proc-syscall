use proc_syscall::syscall;

#[cfg(target_arch = "x86_64")]
pub const SYS_EXIT: usize = 1;

#[test]
fn test_sys_exit() {
    #[syscall(SYS_EXIT)]
    pub type SysExit = fn(code: i32) -> !;
}
