use solana_program::entrypoint::SUCCESS;

#[cfg(not(feature = "asm_return"))]
#[inline(always)]
pub fn return_success() -> u64 {
    SUCCESS
}

#[cfg(feature = "asm_return")]
#[inline(always)]
pub fn return_success() {}

#[cfg(not(feature = "asm_return"))]
#[cold]
#[inline(never)]
pub fn return_error() -> u64 {
    1
}

#[cfg(feature = "asm_return")]
#[cold]
#[inline(never)]
pub fn return_error() {
    sbpf_asm_macros::set_return_imm!(1);
}
