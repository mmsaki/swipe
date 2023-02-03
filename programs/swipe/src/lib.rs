use anchor_lang::prelude::*;
pub mod constant;
pub mod states;
use crate::{constant::*, states::*};

declare_id!("DntPpuc8ea7jLLB6CbHSwzEaAtkHKGTuNwiW57tNRXN6");

// create ad pda
// what is pda? -> a profile derived account
// accouts created from the solana program

#[program]
pub mod swipe {
    use super::*;
    // should create a user account with default data
    pub fn create_user(ctx: Context<CreateUser>, username: String, uri: String) -> Result<()> {
        let user = &mut ctx.accounts.user;
        user.owner = ctx.accounts.owner.key();
        user.last_post = 0;
        user.matches = 0;
        user.username = username;
        user.uri = uri;
        Ok(())
    }
    pub fn update_user(ctx: Context<UpdateUser>, username: String, uri: String) -> Result<()> {
        let user = &mut ctx.accounts.user;
        user.username = username;
        user.uri = uri;

        Ok(())
    }

    pub fn create_post(ctx: Context<CreatePost>, title: String, image: String) -> Result<()> {
        // initialize a tinder account with info passed in
        let post_account = &mut ctx.accounts.post_account;
        let user = &mut ctx.accounts.user;

        post_account.owner = ctx.accounts.owner.key();
        post_account.id = user.last_post;
        post_account.title = title;
        post_account.image = image;

        // increase tinder post index for pda
        user.last_post = user.last_post.checked_add(1).unwrap();

        Ok(())
    }

    pub fn update_post(ctx: Context<UpdatePost>, title: String, image: String) -> Result<()> {
        let post_account = &mut ctx.accounts.post_account;
        post_account.title = title;
        post_account.image = image;

        Ok(())
    }

    pub fn delete_post(_ctx: Context<DeletePost>) -> Result<()> {
        Ok(())
    }

    pub fn like_post(ctx: Context<LikePost>, _post: Pubkey) -> Result<()> {
        let match_account = &mut ctx.accounts.match_account;
        let user = &mut ctx.accounts.user;
        let post = &mut ctx.accounts.post;

        match_account.owner = ctx.accounts.owner.key();
        match_account.user = post.owner;
        match_account.image = post.image.to_string();
        match_account.image = post.title.to_string();
        user.matches += 1;

        Ok(())
    }

    pub fn unmatch(_ctx: Context<Unmatch>) -> Result<()> {
        // unmatch is handled in context
        Ok(())
    }
}

// create the pda context
#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 32 + 32 + 4 + 256 + 4 + 2048,
        seeds = [USER_TAG, owner.key().as_ref()],
        bump
    )]
    pub user: Box<Account<'info, User>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateUser<'info> {
    #[account(
        mut, 
        seeds = [USER_TAG, owner.key().as_ref()],
        bump,
        has_one = owner,
    )]
    pub user: Box<Account<'info, User>>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct CreatePost<'info> {
    #[account(
        mut, 
        seeds = [USER_TAG, owner.key().as_ref()],
        bump,
        has_one = owner,
    )]
    pub user: Box<Account<'info, User>>,

    #[account(
        init, 
        seeds = [POST_TAG, owner.key().as_ref(), &[user.last_post]],
        payer = owner, 
        bump,
        space = 8 + 32 + 32 + 4 + 256 + 4 + 2048,
    )]
    pub post_account: Box<Account<'info, Post>>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id:u8)]
pub struct UpdatePost<'info> {
    #[account(
        mut,
        seeds = [POST_TAG, owner.key().as_ref(), &[id].as_ref()],
        bump,
        has_one = owner,
    )]
    pub post_account: Box<Account<'info, Post>>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id:u8)]
pub struct DeletePost<'info> {
    #[account(
        mut,
        close = owner,
        seeds = [POST_TAG, owner.key().as_ref(), &[id].as_ref()],
        bump,
        has_one = owner,
    )]
    pub post_account: Box<Account<'info, Post>>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(post:Pubkey)]
pub struct LikePost<'info> {
    #[account(
        mut, 
        seeds = [USER_TAG, owner.key().as_ref()],
        bump,
        has_one = owner,
    )]
    pub user: Box<Account<'info, User>>,
    #[account()]
    pub post: Box<Account<'info, Post>>,

    #[account(
        init,
        seeds = [MATCH_TAG, owner.key().as_ref(), post.key().as_ref()],
        bump,
        payer = owner,
        space = 8 + 32 + 32 + 4 + 256 + 4 + 2048,
    )]
    pub match_account: Box<Account<'info, Match>>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Unmatch<'info> {
    #[account(
        mut,
        close = owner,
        seeds = [MATCH_TAG, owner.key().as_ref()],
        bump,
        has_one = owner,
    )]
    pub match_account: Box<Account<'info, Match>>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
