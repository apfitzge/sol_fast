/// Read instruction data slice from the input buffer.
/// This should be called immediately after reading accounts.
#[macro_export]
macro_rules! read_instruction_data {
    ($input:ident, $offset:ident) => {{
        let data_len = unsafe { core::ptr::read($input.add($offset) as *const u64) as usize };
        $offset += core::mem::size_of::<u64>();
        let data = unsafe { core::slice::from_raw_parts($input.add($offset), data_len) };
        $offset += data_len;
        data
    }};
}
