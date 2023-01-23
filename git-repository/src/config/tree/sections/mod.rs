#![allow(missing_docs)]

/// The `author` top-level section.
#[derive(Copy, Clone, Default)]
pub struct Author;
mod author;

/// The `branch` top-level section.
#[derive(Copy, Clone, Default)]
pub struct Branch;
pub mod branch;

/// The `checkout` top-level section.
#[derive(Copy, Clone, Default)]
pub struct Checkout;
pub mod checkout;

/// The `committer` top-level section.
#[derive(Copy, Clone, Default)]
pub struct Committer;
mod committer;

/// The `gitoxide` top-level section.
#[derive(Copy, Clone, Default)]
pub struct Gitoxide;
pub mod gitoxide;

/// The `remote` top-level section.
#[derive(Copy, Clone, Default)]
pub struct Remote;
mod remote;

/// The `user` top-level section.
#[derive(Copy, Clone, Default)]
pub struct User;
mod user;
