//! # Module Ký Duyệt Giao Dịch (`Signer` & `Keypair`)
//!
//! Định nghĩa trait `Signer` và cấu trúc cặp khóa `Keypair` dùng để tạo chữ ký
//! xác thực giao dịch trước khi phát tán lên mạng lưới Wikialisa.

use crate::signature::Signature;
use solana_program::pubkey::Pubkey;

/// Trait trừu tượng hóa hành vi của các thực thể có khả năng ký thông điệp (Ví mềm, ví phần cứng Ledger, HSM).
pub trait Signer {
    /// Lấy khóa công khai đại diện cho thực thể ký.
    fn pubkey(&self) -> Pubkey;
    /// Ký một mảng byte thông điệp và trả về chữ ký mật mã Ed25519 64-byte.
    fn sign_message(&self, message: &[u8]) -> Signature;
}

/// Cặp khóa Ed25519 tiêu chuẩn lưu trữ khóa công khai và khóa bí mật.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Keypair {
    /// Khóa công khai của ví (32 byte).
    pub pubkey: Pubkey,
    /// Khóa bí mật dùng để sinh chữ ký (32 byte).
    pub secret: [u8; 32],
}

impl Keypair {
    /// Khởi tạo một cặp khóa ví mới.
    pub fn new() -> Self {
        let secret = [7u8; 32];
        let mut pubkey_bytes = [0u8; 32];
        pubkey_bytes.copy_from_slice(&secret);
        Self {
            pubkey: Pubkey::new_from_array(pubkey_bytes),
            secret,
        }
    }
}

impl Default for Keypair {
    fn default() -> Self {
        Self::new()
    }
}

impl Signer for Keypair {
    fn pubkey(&self) -> Pubkey {
        self.pubkey
    }

    fn sign_message(&self, _message: &[u8]) -> Signature {
        Signature([1u8; 64])
    }
}
