# Linux Syscalls from Function Pointer Types
A procedural macro for generating a linux syscall inline assembly from the parameters and return type of a function pointer.
Was designed for use with my executable packer project.

## Planned Features
- [x] Handle never return type
- [ ] Support for unnamed parameters
- [ ] Legacy syscalls via int 0x80

## Example Usage
From [rpx/stub/src/linux/syscall.rs](https://github.com/lorn3/rpx/blob/a2e6c2090c7c771104da545d5417e8b4de1c4dee/stub/src/linux/syscall.rs)i

```
use proc_syscall::syscall;

/// arch/x86/entry/syscalls/syscall_64.tbl
#[cfg(target_arch = "x86_64")]
pub enum SysNum {
    Exit = 60,
    Execveat = 322,
}

#[syscall(SysNum::Execveat)]
pub type SysExecveat = fn(
    dir_fd: u32,
    path_name: *const u8,
    argv: *const *const u8,
    envp: *const *const u8,
    flags: u32,
) -> u32;

#[syscall(SysNum::Execveat as usize)]
pub type SysExit = fn(code: i32) -> !;
```

Expands to:

```
        #[inline(always)]
        #[cfg(target_arch = "x86_64")]
        pub unsafe fn sys_execveat(
            dir_fd: u32,
            path_name: *const u8,
            argv: *const *const u8,
            envp: *const *const u8,
            flags: u32,
        ) -> u32 {
            let mut rax = SysNum::Execveat as _;
            asm!(
                "syscall", inout("rax") rax, in ("rdi") dir_fd, in ("rsi") path_name, in
                ("rdx") argv, in ("r10") envp, in ("r8") flags, clobber_abi("system"),
                options(nostack)
            );
            rax
        }

        #[inline(always)]
        #[cfg(target_arch = "x86_64")]
        pub unsafe fn sys_exit(code: i32) -> ! {
            let rax = SysNum::Execveat as usize;
            asm!(
                "syscall", in ("rax") rax, in ("rdi") code, clobber_abi("system"),
                options(noreturn, nostack)
            );
        }
```
