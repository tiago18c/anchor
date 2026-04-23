//! Legacy IDL instruction support has been removed in favor of Program Metadata.
//!
//! This module now only provides the IDL build feature for generating IDLs
//! without injecting instructions into programs.
//!
//! The `legacy-idl` feature re-enables the old IDL-on-chain instructions for
//! backward compatibility. It will be removed in a future release.

#[cfg(feature = "legacy-idl")]
mod legacy {
    use crate::prelude::*;

    // The first 8 bytes of an instruction to create or modify the IDL account. This
    // instruction is defined outside the main program's instruction enum, so that
    // the enum variant tags can align with function source order.
    //
    // Sha256(anchor:idl)[..8];
    #[deprecated(note = "Legacy IDL instructions are deprecated. Use Program Metadata instead.")]
    pub const IDL_IX_TAG: u64 = 0x0a69e9a778bcf440;

    #[deprecated(note = "Legacy IDL instructions are deprecated. Use Program Metadata instead.")]
    pub const IDL_IX_TAG_LE: &[u8] = IDL_IX_TAG.to_le_bytes().as_slice();

    // The Pubkey that is stored as the 'authority' on the IdlAccount when the authority
    // is "erased".
    #[deprecated(
        note = "Legacy IDL authority erasure is deprecated. Use Program Metadata instead."
    )]
    pub const ERASED_AUTHORITY: Pubkey = Pubkey::new_from_array([0u8; 32]);

    #[deprecated(note = "IdlInstruction is deprecated. Use Program Metadata instead.")]
    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub enum IdlInstruction {
        // One time initializer for creating the program's idl account.
        Create { data_len: u64 },
        // Creates a new IDL account buffer. Can be called several times.
        CreateBuffer,
        // Appends the given data to the end of the idl account buffer.
        Write { data: Vec<u8> },
        // Sets a new data buffer for the IdlAccount.
        SetBuffer,
        // Sets a new authority on the IdlAccount.
        SetAuthority { new_authority: Pubkey },
        Close,
        // Increases account size for accounts that need over 10kb.
        Resize { data_len: u64 },
    }

    // The account holding a program's IDL. This is stored on chain so that clients
    // can fetch it and generate a client with nothing but a program's ID.
    //
    // Note: we use the same account for the "write buffer", similar to the
    //       bpf upgradeable loader's mechanism.
    #[deprecated(note = "IdlAccount is deprecated. Use Program Metadata instead.")]
    #[account("internal")]
    #[derive(Debug)]
    pub struct IdlAccount {
        // Address that can modify the IDL.
        pub authority: Pubkey,
        // Length of compressed idl bytes.
        pub data_len: u32,
        // Followed by compressed idl bytes.
    }

    #[allow(deprecated)]
    impl IdlAccount {
        pub fn address(program_id: &Pubkey) -> Pubkey {
            let program_signer = Pubkey::find_program_address(&[], program_id).0;
            Pubkey::create_with_seed(&program_signer, IdlAccount::seed(), program_id)
                .expect("Seed is always valid")
        }
        pub fn seed() -> &'static str {
            "anchor:idl"
        }
    }
}

#[cfg(feature = "legacy-idl")]
#[allow(deprecated)]
pub use legacy::*;

#[cfg(feature = "idl-build")]
pub use anchor_lang_idl::{build::IdlBuild, *};
