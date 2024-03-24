use anchor_lang::prelude::*;

declare_id!("FM6FJg5bPtMwAXrvP97tZXQptd5iL2DYjtU52SSCUPeA");

#[program]
pub mod bloxchain {
    use super::*;

    pub fn create_game(ctx: Context<CreateGame>, code: String, name: String, nft_collections: Vec<Pubkey>, tokens: Vec<Pubkey>, description: String, img_src: String) -> Result<()> {
        // we'll create the game account before
        // tokens and nfts are created in the frontend and addresses are passed in
        let game = &mut ctx.accounts.game;
        game.creator = ctx.accounts.signer.key();
        game.code = code;
        game.name = name;
        game.nft_collections = nft_collections;
        game.tokens = tokens;
        game.description = description;
        game.img_src = img_src;
        Ok(())
    }
    pub fn save_game(ctx: Context<SaveGame>, code: String, name: String) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.owner = ctx.accounts.signer.key();
        game.code = code;
        game.name = name;
        Ok(())
    }
    pub fn edit_game(ctx: Context<EditGame>, code: String) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.code = code;
        Ok(())
    }
}

#[account]
pub struct Game {
    code: String,
    name: String,
    creator: Pubkey,
    nft_collections: Vec<Pubkey>,
    tokens: Vec<Pubkey>,
    description: String,
    img_src: String
}

#[account]
pub struct SavedGame {
    code: String,
    owner: Pubkey,
    name: String
}

#[derive(Accounts)]
pub struct CreateGame<'info> {
    pub game: Account<'info, Game>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct SaveGame<'info> {
    pub game: Account<'info, SavedGame>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}
#[derive(Accounts)]
pub struct EditGame<'info> {
    #[account(mut, has_one = creator)]
    pub game: Account<'info, Game>,
    pub creator: Signer<'info>,
}