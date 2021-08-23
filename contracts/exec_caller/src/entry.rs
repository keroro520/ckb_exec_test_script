// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

use alloc::{vec, vec::Vec};

use crate::ckb_std::{high_level::load_script, syscalls::SysError};
use crate::ckb_types::{bytes::Bytes, prelude::*};
use crate::error::Error;
use exec_params::{ExecParams, ExecParamsReader};

pub const SYS_VM_VERSION: u64 = 2041;
pub const SYS_CURRENT_CYCLES: u64 = 2042;
pub const SYS_EXEC: u64 = 2043;


pub fn main() -> Result<(), Error> {
    let vm_version = syscall_vm_version();
    assert_eq!(vm_version, 1);

    let current_cycles = syscall_current_cycles();
    assert!(current_cycles > 0);

    let script = load_script()?;
    let script_args = script.args();
    let script_args_bytes = script_args.as_bytes();
    ExecParamsReader::verify(&script_args_bytes, false)?;

    let exec_params_reader: ExecParamsReader = ExecParamsReader::new_unchecked(&script_args_bytes);
    let exec_params: ExecParams = exec_params_reader.to_entity();
    let source: u32 = exec_params.source().unpack();
    let place: u32 = exec_params.place().unpack();
    let index: u32 = exec_params.index().unpack();
    let bounds: u64 = exec_params.bounds().unpack();
    let argc = 1;
    let expected_result: Bytes = exec_params.expected_result().unpack();
    let argv = vec![expected_result];

    syscall_exec(index, source, place, bounds, argc, argv).map_err(Into::into)
}

fn syscall_vm_version() -> i32 {
    let vm_version = unsafe { syscall(0, 0, 0, 0, 0, 0, 0, SYS_VM_VERSION) };
    vm_version as i32
}

fn syscall_current_cycles() -> i32 {
    let vm_version = unsafe { syscall(0, 0, 0, 0, 0, 0, 0, SYS_CURRENT_CYCLES) };
    vm_version as i32
}

fn syscall_exec(
    index: u32,
    source: u32,
    place: u32,
    bounds: u64,
    argc: i32,
    argv: Vec<Bytes>,
) -> Result<(), SysError> {
    let ret = unsafe {
        syscall(
            index as u64,
            source as u64,
            place as u64,
            bounds,
            argc as u64,
            argv.as_ptr() as u64,
            0,
            SYS_EXEC,
        )
    };

    match ret {
        0 => Ok(()),
        1 => Err(SysError::IndexOutOfBound),
        2 => Err(SysError::ItemMissing),
        _ => Err(SysError::Unknown(ret)),
    }
}

#[cfg(target_arch = "riscv64")]
#[link(name = "ckb-syscall")]
extern "C" {
    fn syscall(a0: u64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64, a7: u64) -> u64;
}

#[cfg(not(target_arch = "riscv64"))]
fn syscall(a0: u64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64, a7: u64) -> u64 {
    return u64::MAX;
}
