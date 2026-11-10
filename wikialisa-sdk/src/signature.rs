//! # Module Chữ Ký Mật Mã Học (`Signature`)
//!
//! Định nghĩa cấu trúc chữ ký 64-byte Ed25519 dùng để chứng thực tính toàn vẹn
//! và ủy quyền giao dịch trên mạng Wikialisa.

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Chữ ký mật mã học Ed25519 độ dài cố định 64 byte.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct Signature(pub [u8; 64]);

impl Signature {
    /// Khởi tạo một chữ ký mới từ mảng 64 byte thô.
    pub const fn new(signature_bytes: [u8; 64]) -> Self {
        Self(signature_bytes)
    }

    /// Trả về tham chiếu đến mảng 64 byte của chữ ký.
    pub fn as_ref(&self) -> &[u8; 64] {
        &self.0
    }
}

impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Signature({})", hex::encode(&self.0[..8]))
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
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
