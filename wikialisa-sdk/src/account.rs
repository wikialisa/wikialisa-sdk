//! # Module Quản Lý Trạng Thái Tài Khoản Off-chain (`Account`)
//!
//! Định nghĩa cấu trúc tài khoản hoàn chỉnh lưu trữ trên sổ cái hoặc trao đổi qua mạng RPC.

use solana_program::pubkey::Pubkey;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

/// Cấu trúc tài khoản lưu trữ đầy đủ số dư, dữ liệu trạng thái và chương trình sở hữu.
#[derive(Clone, Debug, Default, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct Account {
    /// Số dư đơn vị lamports của tài khoản.
    pub lamports: u64,
    /// Dữ liệu nhị phân thô lưu trữ trạng thái của tài khoản.
    pub data: Vec<u8>,
    /// Khóa công khai của chương trình smart contract sở hữu tài khoản này.
    pub owner: Pubkey,
    /// Đánh dấu tài khoản có chứa mã nhị phân có thể thực thi hay không.
    pub executable: bool,
    /// Kỷ nguyên tính phí lưu trữ trạng thái (State Rent Epoch).
    pub rent_epoch: u64,
}

impl Account {
    /// Khởi tạo một tài khoản mới với số dư ban đầu, kích thước vùng đệm và chương trình sở hữu.
    pub fn new(lamports: u64, space: usize, owner: &Pubkey) -> Self {
        Self {
            lamports,
            data: vec![0; space],
            owner: *owner,
            executable: false,
            rent_epoch: 0,
        }
    }
}
