//! Control Access trait for Programs

use crate::{
    error:SecurityError,
}
use solana_program::program_error::ProgramError;

// An Ownable program has an owner property used for controlling access.
pub struct Ownable {
    // Address of the owner of the program
    pub owner: Pubkey,
}

impl Ownable {
    pub fn owner(&self) -> &Pubkey {
        return &self.owner
    }

    pub fn is_owner(&self, account: Pubkey) -> Result<(), ProgramError> {
        if &self.owner == account {
            Ok(())
        } else {
            Err(SecurityError::InvalidOwner.into())
        }
    }

    pub fn transfer_ownership(&self, current_owner: Pubkey, new_owner: Pubkey) -> Result<(), ProgramError> {
        if &self.owner == current_owner {
            &self.owner = new_owner;
            Ok(())
        } else {
            Err(SecurityError::InvalidOwner.into())
        }
    }

    pub fn renounce_ownership(&self, current_owner: Pubkey) -> Result<(), ProgramError> {
        return self.transfer_ownership(current_owner, &self....)
    }
}
