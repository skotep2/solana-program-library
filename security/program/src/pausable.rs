//! Execution Control trait for Programs

use solana_program::program_error::ProgramError;

// A Pausable program can have its execution paused by its owner
pub struct Pausable {
    // Is the program paused or not
    pub paused: bool
}

impl Pausable {
    pub fn paused(&self) -> bool {
        return &self.paused;
    }

    pub fn pause(&self, account: Pubkey) -> Result<(), ProgramError> {
        match &self.is_owner(account)) {
            Err(why) => Err(why),
            Ok() => {
                &self.pause = true;
                Ok(());
            }
        }
    }

    pub fn resume(&self, account: Pubkey) -> Result<(), ProgramError> {
        match &self.is_owner(account)) {
            Err(why) => Err(why),
            Ok() => {
                &self.pause = false;
                Ok(());
            }
        }
    }
}

