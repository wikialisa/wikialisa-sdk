//! # Bộ Công Cụ Phát Triển Phần Mềm Wikialisa (`wikialisa-sdk`)
//!
//! Thư viện trung tâm cung cấp các cấu trúc giao dịch, chữ ký mật mã học,
//! cơ chế ký duyệt và các bí danh tương thích cho lập trình viên phát triển
//! ứng dụng phi tập trung (dApps) và công cụ off-chain trên **Wikialisa Layer 1**.
//!
//! Đồng thời tái xuất khẩu toàn bộ module `wikialisa-program` để đảm bảo trải nghiệm
//! lập trình đồng nhất và ngăn ngừa xung đột namespace giữa các crate.

pub use solana_program as program;
pub use solana_program::{
    account_info, entrypoint, instruction, msg, program_error, pubkey,
    AccountInfo, AccountMeta, Instruction, ProgramError, ProgramResult, Pubkey,
};

pub mod account;
pub mod signature;
pub mod signer;
pub mod transaction;

pub use account::Account;
pub use signature::Signature;
pub use signer::Signer;
pub use transaction::Transaction;
