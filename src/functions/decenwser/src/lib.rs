use anchor_lang::prelude::*;
use instructions::*;
use crate::error::ErrorCode;


pub mod error;
pub mod instructions;
pub mod state;

declare_id!("A9KnHa1iaHtDfsZDNNnM8ZMSmBr6gBFqMHnqw7EyyPir");

#[program]
pub mod decenwser {
    use super::*;
    pub fn main_account(
        ctx: Context<MainAccountStruct>,
        web_name: String
    ) -> Result<()> {
        instructions::main_account::main_account(
            ctx,
            web_name
        )
    }
    pub fn html_store(
        ctx: Context<HtmlStore>,
        content: String,
    ) -> Result<()> {
        instructions::html_store::html_store(
            ctx,
            content,
        )
    }
    pub fn js_store(
        ctx: Context<JsStore>,
        content: String,
    ) -> Result<()> {
        instructions::js_store::js_store(
            ctx,
            content,
        )
    }
    pub fn delete_html(
        ctx: Context<DeleteHtml>,
    ) -> Result<()> {
        instructions::delete_html::delete_html(
            ctx,
        )
    }
    pub fn delete_js(
        ctx: Context<DeleteJs>,
    ) -> Result<()> {
        instructions::delete_js::delete_js(
            ctx,
        )
    }
    pub fn speed_html_store(
        ctx: Context<SpeedHtmlStore>,
        content: String,
        len: usize,
    ) -> Result<()> {
        instructions::speed_html_store::speed_html_store(
            ctx,
            content,
            len,
        )
    }
    pub fn speed_js_store(
        ctx: Context<SpeedJsStore>,
        content: String,
        len: usize,
    ) -> Result<()> {
        instructions::speed_js_store::speed_js_store(
            ctx,
            content,
            len,
        )
    }
}