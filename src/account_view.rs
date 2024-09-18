// TODO: use `solana_pubkey` when published
use solana_program::pubkey::Pubkey;

/// Read the number of accounts.
/// # Safety
/// - `input` must be a valid pointer to the start of the input buffer.
/// - `offset` should be 0.
#[inline]
pub unsafe fn read_num_accounts(input: *mut u8, offset: &mut usize) -> u64 {
    let num_accounts = core::ptr::read(input.add(*offset) as *const u64);
    *offset += core::mem::size_of::<u64>();
    num_accounts
}

/// # Safety
/// - `input` must be a valid pointer to the start of the input buffer.
/// - `offset` should be offset after reading the number of accounts
///   or reading up to the number of accounts.
#[inline]
pub unsafe fn read_account_view(input: *mut u8, offset: &mut usize) -> ReadAccountView {
    // Get the pointer to the current offset, store in the `AccountView`.
    let account_view = AccountView {
        ptr: unsafe { input.add(*offset) },
    };

    // Here, we must read the duplicate flag.
    // This is because the account is serialized differently
    // (i.e. skipped) if the account is a duplicate.
    // In those cases, we will return the duplicate index;
    // the caller is responsible for handling the mapping of these
    // indexes, should they need to.
    let dup = account_view.duplicate();
    if dup != solana_program::entrypoint::NON_DUP_MARKER {
        #[allow(unused_assignments)]
        {
            *offset += solana_program::entrypoint::BPF_ALIGN_OF_U128; // Update offset to just be at next 8-byte alignment
        }
        ReadAccountView::Duplicate(dup)
    } else {
        // Update offset to:
        // 1. Skip static fields up to the data
        // 2. Read the data len, and skip data
        // 3. Add the max allowed increase in data size
        // 4. Update for alignment
        #[allow(unused_assignments)]
        {
            *offset += AccountView::DATA_OFFSET;
            *offset += account_view.data_len() as usize;
            *offset += solana_program::entrypoint::MAX_PERMITTED_DATA_INCREASE;
            *offset +=
                (*offset as *const u8).align_offset(solana_program::entrypoint::BPF_ALIGN_OF_U128);
            *offset += core::mem::size_of::<u64>(); // rent epoch (deprecated)
        }

        ReadAccountView::View(account_view)
    }
}

/// An account view read by `account_view!` macro, OR
/// the index of the account this is a duplicate of.
pub enum ReadAccountView {
    View(AccountView),
    Duplicate(u8),
}

/// Accessor for account data.
pub struct AccountView {
    /// Pointer to the start of the account.
    pub ptr: *mut u8,
}

impl AccountView {
    // Account Layout is as follows:
    // duplicate - u8
    pub const DUPLICATE_OFFSET: usize = 0;
    // is_signed - u8
    pub const IS_SIGNED_OFFSET: usize = Self::DUPLICATE_OFFSET + core::mem::size_of::<u8>();
    // is_writable - u8
    pub const IS_WRITABLE_OFFSET: usize = Self::IS_SIGNED_OFFSET + core::mem::size_of::<u8>();
    // executable - u8
    pub const EXECUTABLE_OFFSET: usize = Self::IS_WRITABLE_OFFSET + core::mem::size_of::<u8>();
    // original_data_len - u32
    pub const ORIGINAL_DATA_LEN_OFFSET: usize =
        Self::EXECUTABLE_OFFSET + core::mem::size_of::<u8>();
    // pubkey - Pubkey
    pub const PUBKEY_OFFSET: usize = Self::ORIGINAL_DATA_LEN_OFFSET + core::mem::size_of::<u32>();
    // owner - Pubkey
    pub const OWNER_OFFSET: usize = Self::PUBKEY_OFFSET + core::mem::size_of::<Pubkey>();
    // lamports - u64
    pub const LAMPORTS_OFFSET: usize = Self::OWNER_OFFSET + core::mem::size_of::<Pubkey>();
    // data_len - u64
    pub const DATA_LEN_OFFSET: usize = Self::LAMPORTS_OFFSET + core::mem::size_of::<u64>();
    // data - u8[]
    pub const DATA_OFFSET: usize = Self::DATA_LEN_OFFSET + core::mem::size_of::<u64>();

    /// Copy the duplicate field.
    /// # Safety
    /// - `AccountView` was initialized with a valid pointer.
    #[inline]
    pub unsafe fn duplicate(&self) -> u8 {
        core::ptr::read(self.ptr.add(Self::DUPLICATE_OFFSET))
    }

    /// Copy the is_signed field.
    /// # Safety
    /// - `AccountView` was initialized with a valid pointer.
    #[inline]
    pub unsafe fn is_signed(&self) -> u8 {
        core::ptr::read(self.ptr.add(Self::IS_SIGNED_OFFSET))
    }

    /// Copy the is_writable field.
    /// # Safety
    /// - `AccountView` was initialized with a valid pointer.
    #[inline]
    pub unsafe fn is_writable(&self) -> u8 {
        core::ptr::read(self.ptr.add(Self::IS_WRITABLE_OFFSET))
    }

    /// Copy the executable field.
    /// # Safety
    /// - `AccountView` was initialized with a valid pointer.
    #[inline]
    pub unsafe fn executable(&self) -> u8 {
        core::ptr::read(self.ptr.add(Self::EXECUTABLE_OFFSET))
    }

    /// Get a reference to the pubkey.
    /// # Safety
    /// - `AccountView` was initialized with a valid pointer.
    #[inline]
    pub unsafe fn pubkey(&self) -> &Pubkey {
        &*(self.ptr.add(Self::PUBKEY_OFFSET) as *const Pubkey)
    }

    /// Get a reference to the owner.
    /// # Safety
    /// - `AccountView` was initialized with a valid pointer.
    #[inline]
    pub unsafe fn owner(&self) -> &Pubkey {
        &*(self.ptr.add(Self::OWNER_OFFSET) as *const Pubkey)
    }

    /// Copy the lamports field.
    /// # Safety
    /// - `AccountView` was initialized with a valid pointer.
    #[inline]
    pub unsafe fn lamports(&self) -> u64 {
        core::ptr::read(self.ptr.add(Self::LAMPORTS_OFFSET) as *const u64)
    }

    /// Get a mutable reference to the lamports field.
    /// # Safety
    /// - `AccountView` was initialized with a valid pointer.
    #[inline]
    pub unsafe fn lamports_mut(&mut self) -> &mut u64 {
        &mut *(self.ptr.add(Self::LAMPORTS_OFFSET) as *mut u64)
    }

    /// Copy the data_len field.
    /// # Safety
    /// - `AccountView` was initialized with a valid pointer.
    #[inline]
    pub unsafe fn data_len(&self) -> u64 {
        core::ptr::read(self.ptr.add(Self::DATA_LEN_OFFSET) as *const u64)
    }

    /// Get a mutable reference to the data_len field.
    /// # Safety
    /// - `AccountView` was initialized with a valid pointer.
    /// - data len should not be modified to exceed the maximum data size.
    #[inline]
    pub unsafe fn data_len_mut(&mut self) -> &mut u64 {
        &mut *(self.ptr.add(Self::DATA_LEN_OFFSET) as *mut u64)
    }

    /// Get a reference to the data.
    /// # Safety
    /// - `AccountView` was initialized with a valid pointer.
    #[inline]
    pub unsafe fn data(&self) -> &[u8] {
        core::slice::from_raw_parts(self.ptr.add(Self::DATA_OFFSET), self.data_len() as usize)
    }

    /// Get a mutable reference to the data.
    /// # Safety
    /// - `AccountView` was initialized with a valid pointer.
    #[inline]
    pub unsafe fn data_mut(&mut self) -> &mut [u8] {
        core::slice::from_raw_parts_mut(self.ptr.add(Self::DATA_OFFSET), self.data_len() as usize)
    }
}
