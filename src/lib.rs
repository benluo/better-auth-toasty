// SPDX-License-Identifier: MIT

//! # better-auth-toasty
//!
//! A database adapter bridging `better-auth-rs` and the `toasty` ORM.
//!
//! This allows you to use Toasty-supported databases (like Turso/SQLite, DynamoDB)
//! to back your Better Auth authentication system.

pub mod adapter;
mod conversions;
pub mod models;
mod ops;

pub use adapter::ToastyAdapter;
