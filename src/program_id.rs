#[macro_export]
macro_rules! read_program_id {
    ($input:ident, $offset:ident) => {{
        let program_id =
            unsafe { &*($input.add($offset) as *const solana_program::pubkey::Pubkey) };
        $offset += core::mem::size_of::<solana_program::pubkey::Pubkey>();
        program_id
    }};
}
