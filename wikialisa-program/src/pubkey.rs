//! # Module Khóa Công Khai Định Danh (`Pubkey`)
//!
//! Định nghĩa cấu trúc địa chỉ tài khoản 32-byte tiêu chuẩn mật mã học trên Wikialisa,
//! hỗ trợ chuyển đổi chuỗi ký tự hexa, base58 và tương thích tuần tự hóa Borsh/Serde.

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Cấu trúc khóa công khai 32-byte (256-bit) đại diện cho địa chỉ tài khoản trên Wikialisa.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct Pubkey(pub [u8; 32]);

impl Pubkey {
    /// Khởi tạo một địa chỉ `Pubkey` mới từ một mảng 32 byte thô.
    pub const fn new_from_array(array: [u8; 32]) -> Self {
        Self(array)
    }

    /// Trả về mảng 32 byte của địa chỉ khóa công khai.
    pub fn to_bytes(self) -> [u8; 32] {
        self.0
    }

    /// Trả về tham chiếu mảng 32 byte thô.
    pub fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Debug for Pubkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pubkey({})", hex::encode(self.0))
    }
}

impl fmt::Display for Pubkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl From<[u8; 32]> for Pubkey {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

impl AsRef<[u8]> for Pubkey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

mod hex {
    pub fn encode<T: AsRef<[u8]>>(data: T) -> String {
        data.as_ref()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    }
}
