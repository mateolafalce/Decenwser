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
        content: Vec<u8>,
    ) -> Result<()> {
        instructions::html_store::html_store(
            ctx,
            content,
        )
    }
    pub fn js_store(
        ctx: Context<JsStore>,
        content: Vec<u8>,
    ) -> Result<()> {
        instructions::js_store::js_store(
            ctx,
            content,
        )
    }
    pub fn add_html(
        ctx: Context<AddHtml>,
        content: Vec<u8>,
    ) -> Result<()> {
        instructions::add_html::add_html(
            ctx,
            content,
        )
    }
    pub fn add_js(
        ctx: Context<AddJs>,
        content: Vec<u8>,
    ) -> Result<()> {
        instructions::add_js::add_js(
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
}