/// Read the number of accounts from the input buffer.
/// # Safety
/// - `input` must be a valid pointer to the start of the input buffer.
pub unsafe fn read_num_accounts(input: *mut u8, offset: &mut usize) -> u64 {
    let num_accounts = unsafe { core::ptr::read(input.add(*offset) as *const u64) };
    *offset += core::mem::size_of::<u64>();
    num_accounts
}

/// Read a `num_accounts` accounts into an array of `account_limit`.
/// This macro should be used immediately after `read_num_accounts`.
#[macro_export]
macro_rules! read_accounts_no_dup {
    ($input:ident, $offset:ident, $num_accounts:expr, $account_limit:expr) => {{
        // These pointers are uninitialized, but we will initialize them in the loop.
        // If they are not initialized, this is fine because we will not read from them,
        // at least in the macros provided by this crate.
        #[allow(invalid_value)]
        let mut accounts: [$crate::account_view::AccountView; $account_limit] =
            unsafe { core::mem::MaybeUninit::uninit().assume_init() };

        for account in accounts.iter_mut() {
            *account = match $crate::account_view::read_account_view($input, &mut $offset) {
                $crate::account_view::ReadAccountView::View(account) => account,
                _ => return 1,
            };
        }

        accounts
    }};
}
