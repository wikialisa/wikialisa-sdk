//! # Thư Viện Smart Contract Lõi Wikialisa (`wikialisa-program`)
//! 
//! Cung cấp các cấu trúc dữ liệu nguyên thủy (primitives), macro và giao diện cơ sở 
//! để lập trình các hợp đồng thông minh chạy trực tiếp trên chuỗi (on-chain programs)
//! của mạng lưới blockchain **Wikialisa Layer 1**.
//!
//! Kế thừa và tối ưu hóa từ nền tảng Solana SDK với sự tinh gọn về dung lượng mã máy
//! và khả năng tương thích cao.

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

pub mod account_info;
pub mod entrypoint;
pub mod instruction;
pub mod msg;
pub mod program_error;
pub mod pubkey;

pub use account_info::AccountInfo;
pub use entrypoint::ProgramResult;
pub use instruction::{AccountMeta, Instruction};
pub use program_error::ProgramError;
pub use pubkey::Pubkey;

/// Macro ghi log chẩn đoán nội bộ dành cho các chương trình smart contract trên Wikialisa.
/// 
/// Khi thực thi trên máy ảo on-chain, macro này sẽ ghi dữ liệu nhật ký vào vùng đệm 
/// log của node để người dùng và dApp có thể theo dõi tiến trình thực thi.
#[macro_export]
macro_rules! msg {
    ($($arg:tt)*) => {
        println!("[wikialisa-program] {}", format!($($arg)*));
    };
}
