/// Provide an entrypoint for the program.
#[macro_export]
macro_rules! entrypoint_no_dup {
    ($process_instruction:ident, $num_account_validation:ident, $account_limit:expr) => {
        #[no_mangle]
        pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
            let mut offset = 0;
            let num_accounts = $crate::accounts::read_num_accounts(input, &mut offset);
            if !$num_account_validation(num_accounts) {
                return 1;
            }

            let mut accounts =
                $crate::read_accounts_no_dup!(input, offset, num_accounts, $account_limit);
            let instruction_data = $crate::read_instruction_data!(input, offset);
            let program_id = $crate::read_program_id!(input, offset);

            return $process_instruction(
                &mut accounts[..num_accounts as usize],
                instruction_data,
                program_id,
            );
        }
    };
}
