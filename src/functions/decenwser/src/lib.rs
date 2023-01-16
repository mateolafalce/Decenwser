use anchor_lang::prelude::*;

declare_id!("4CXvM9ENhCMGsfz7YPjqDjAkqwLqMwTvw3SBq3YChBNN");

#[account]
pub struct MainAccount {
    pub bump_original: u8,      // 1
    pub authority: Pubkey,      // 32
    pub web_name: String,       // 4 + 32
    pub html: Vec<u64>,         // 4 + 8
    pub css: Vec<u64>,          // 4 + 8
    pub js: Vec<u64>,           // 4 + 8
    pub len: u16,               // 2
}
#[account]
pub struct HTML {
    pub html: String
}
#[account]
pub struct CSS {
    pub css: String      
}
#[account]
pub struct JS {
    pub js: String    
}