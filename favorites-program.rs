use anchor_lang::prelude::*;

declare_id!("ANJbjjMzrffHwaLTgD5MY4CKdQ8BN6hDnhrLHMnX9H2M");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favourites {
    use super::*;

    pub fn set_favourites(
        context: Context<SetFavorites>, 
        number: u64, 
        color: String, 
        hobbies: Vec<String>,
    ) -> Result<()> {
        msg!(
            "{}'s favorite number is {}, favorite color is {}, and their hobbies are {:?}",
            context.accounts.user.key(),
            number,
            color,
            hobbies
        );

        context.accounts.favourites.set_inner(Favourites {
            number,
            color,
            hobbies,
        });

        Ok(())
    }
}

#[account]
pub struct Favourites {
    pub number: u64,
    pub color: String, // max length: 50
    pub hobbies: Vec<String>, // max length: 5 strings of 50 chars
}

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + 8 + 4 + 50 + 4 + (5 * 50),
        seeds = [b"favorites", user.key().as_ref()],
        bump
    )]
    pub favourites: Account<'info, Favourites>,

    pub system_program: Program<'info, System>
}
