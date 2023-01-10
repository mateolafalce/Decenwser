use anchor_lang::prelude::*;

declare_id!("9pSAYQj6tgJCv4Msii77smfYoY3sQGGLf89qnH9EBWry");

#[account]
pub struct MainAccount {
    pub bump_original: u8, // 1
    pub authority: Pubkey, // 32
    pub web_name: String,  // 4 + 32
    pub html: Vec<String>, // 4 + 88
    pub css: Vec<String>,  // 4 + 88
    pub js: Vec<String>,   // 4 + 88
    pub len: u16,          // 2
}
