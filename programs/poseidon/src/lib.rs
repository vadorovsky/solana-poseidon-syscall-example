use anchor_lang::{prelude::*, solana_program::poseidon::hashv};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod poseidon {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        let input1 = [0u8; 32];
        let input2 = [1u8; 32];

        let poseidon = hashv(&[&input1, &input2]);
        msg!("Poseidon hash: {:?}", poseidon);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
