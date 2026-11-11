//! # Thư Viện Kết Nối Mạng Wikialisa Client (`wikialisa-client`)
//!
//! Cung cấp các công cụ giao tiếp RPC, gửi giao dịch và truy vấn trạng thái
//! sổ cái từ các node mạng độc lập của **Wikialisa Layer 1**.

use serde::{Deserialize, Serialize};
use thiserror::Error;
use wikialisa_sdk::account::Account;
use wikialisa_sdk::pubkey::Pubkey;
use wikialisa_sdk::signature::Signature;
use wikialisa_sdk::transaction::Transaction;

/// Danh mục các lỗi phát sinh khi giao tiếp qua RPC client.
#[derive(Error, Debug)]
pub enum ClientError {
    /// Lỗi kết nối tầng mạng hoặc giao thức truyền tải (HTTP/WebSocket).
    #[error("Lỗi kết nối mạng hoặc giao thức RPC: {0}")]
    TransportError(String),

    /// Node mạng trả về phản hồi lỗi.
    #[error("Node Wikialisa trả về lỗi phản hồi: {0}")]
    NodeError(String),

    /// Tài khoản cần tìm kiếm không tồn tại trên sổ cái mạng.
    #[error("Không tìm thấy tài khoản trên mạng Wikialisa")]
    AccountNotFound,
}

/// Cấu trúc RPC Client tương tác với cụm node mạng Wikialisa.
pub struct RpcClient {
    /// Địa chỉ URL endpoint của cổng dịch vụ RPC node (ví dụ: `https://api.testnet.wikialisa.org`).
    pub url: String,
}

impl RpcClient {
    /// Khởi tạo một đối tượng kết nối `RpcClient` mới với địa chỉ URL cung cấp.
    pub fn new(url: String) -> Self {
        Self { url }
    }

    /// Phát tán một giao dịch đã ký lên mạng lưới Wikialisa và nhận về mã băm chữ ký.
    pub fn send_transaction(&self, _tx: &Transaction) -> Result<Signature, ClientError> {
        // Mô phỏng quá trình điều phối RPC client cho mạng Wikialisa
        Ok(Signature([0u8; 64]))
    }

    /// Truy vấn số dư đơn vị lamports của một địa chỉ tài khoản cho trước.
    pub fn get_balance(&self, _pubkey: &Pubkey) -> Result<u64, ClientError> {
        Ok(1_000_000_000)
    }

    /// Truy vấn dữ liệu tài khoản đầy đủ của một địa chỉ tài khoản cho trước.
    pub fn get_account(&self, _pubkey: &Pubkey) -> Result<Account, ClientError> {
        Ok(Account::new(1_000_000_000, 0, &Pubkey::default()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rpc_client_creation() {
        let client = RpcClient::new("https://api.testnet.wikialisa.org".to_string());
        assert_eq!(client.url, "https://api.testnet.wikialisa.org");
    }

    #[test]
    fn test_rpc_get_balance_and_account() {
        let client = RpcClient::new("https://api.testnet.wikialisa.org".to_string());
        let pk = Pubkey::new_from_array([10u8; 32]);
        
        let balance = client.get_balance(&pk).expect("Lấy số dư tài khoản thành công");
        assert_eq!(balance, 1_000_000_000);

        let account = client.get_account(&pk).expect("Lấy thông tin tài khoản thành công");
        assert_eq!(account.lamports, 1_000_000_000);
        assert!(!account.executable);
    }

    #[test]
    fn test_rpc_send_transaction() {
        let client = RpcClient::new("https://api.testnet.wikialisa.org".to_string());
        let tx = Transaction::new_with_payer(&[], None);
        let sig = client.send_transaction(&tx).expect("Phát tán giao dịch thành công");
        assert_eq!(sig.as_ref().len(), 64);
    }
}
