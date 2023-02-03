use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct User {
    pub owner: Pubkey,    // 32
    pub username: String, // 4 + 256
    pub last_post: u8,    // 1
    pub matches: u32,     // 32
    pub uri: String,      // 4 + 2048
}

#[account]
#[derive(Default)]
pub struct Post {
    pub owner: Pubkey, // 32
    pub id: u8,        // 32
    pub title: String, // 4 + 256
    pub image: String, // 4 + 2048
}

#[account]
#[derive(Default)]
pub struct Match {
    pub owner: Pubkey, // 32
    pub user: Pubkey,  // 32
    pub image: String, // 4 + 2048
    pub title: String, // 4 + 256
}
