use anchor_lang::prelude::*;
use instructions::*;
use crate::error::ErrorCode;


pub mod error;
pub mod instructions;
pub mod state;

declare_id!("5t8N3VyXYMRYvuRQtmTZ3HmmRYK7En9ZHXVLLQZBC4gr");

#[program]
pub mod decenwser {
    use super::*;
    pub fn decenwser(
        ctx: Context<Decenwser>,
    ) -> Result<()> {
        instructions::decenwser::decenwser(
            ctx,
        )
    }
    pub fn html_store(
        ctx: Context<HtmlStore>,
        html: String
    ) -> Result<()> {
        instructions::html_store::html_store(
            ctx,
            html
        )
    }
    pub fn css_store(
        ctx: Context<CssStore>,
        css: String
    ) -> Result<()> {
        instructions::css_store::css_store(
            ctx,
            css
        )
    }
    pub fn js_store(
        ctx: Context<JsStore>,
        js: String
    ) -> Result<()> {
        instructions::js_store::js_store(
            ctx,
            js
        )
    }
    pub fn main_account(
        ctx: Context<MainAccountStruct>,
        web_name: String
    ) -> Result<()> {
        instructions::main_account::main_account(
            ctx,
            web_name
        )
    }
}