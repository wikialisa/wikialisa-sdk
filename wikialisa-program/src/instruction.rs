//! # Module Chỉ Mục Lệnh Giao Dịch (`Instruction`)
//!
//! Định nghĩa cấu trúc chỉ mục lệnh (Instruction) và siêu dữ liệu tài khoản (`AccountMeta`)
//! dùng để gửi thông điệp yêu cầu thực thi tới các smart contract trên Wikialisa.

use crate::pubkey::Pubkey;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

/// Siêu dữ liệu miêu tả một tài khoản tham gia vào một lệnh thực thi.
#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct AccountMeta {
    /// Khóa công khai của tài khoản.
    pub pubkey: Pubkey,
    /// Tài khoản này có bắt buộc phải ký vào giao dịch hay không.
    pub is_signer: bool,
    /// Tài khoản này có bị biến đổi trạng thái dữ liệu/số dư trong quá trình thực thi hay không.
    pub is_writable: bool,
}

impl AccountMeta {
    /// Khởi tạo một `AccountMeta` có quyền ghi (writable) mặc định.
    pub fn new(pubkey: Pubkey, is_signer: bool) -> Self {
        Self {
            pubkey,
            is_signer,
            is_writable: true,
        }
    }

    /// Khởi tạo một `AccountMeta` ở chế độ chỉ đọc (read-only), không cho phép ghi.
    pub fn new_readonly(pubkey: Pubkey, is_signer: bool) -> Self {
        Self {
            pubkey,
            is_signer,
            is_writable: false,
        }
    }
}

/// Cấu trúc lệnh thực thi đơn lẻ gửi tới một smart contract cụ thể.
#[derive(Clone, Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct Instruction {
    /// Khóa định danh của chương trình mục tiêu nhận và xử lý lệnh này.
    pub program_id: Pubkey,
    /// Danh sách các tài khoản liên quan cần nạp vào máy ảo.
    pub accounts: Vec<AccountMeta>,
    /// Mảng byte nhị phân chứa tham số hoặc dữ liệu lệnh theo định dạng Borsh.
    pub data: Vec<u8>,
}

impl Instruction {
    /// Khởi tạo một `Instruction` kèm theo dữ liệu đầu vào được tự động mã hóa bằng chuẩn Borsh.
    pub fn new_with_borsh<T: BorshSerialize>(
        program_id: Pubkey,
        data: &T,
        accounts: Vec<AccountMeta>,
    ) -> Result<Self, std::io::Error> {
        let mut buffer = Vec::new();
        borsh::BorshSerialize::serialize(data, &mut buffer)?;
        Ok(Self {
            program_id,
            accounts,
            data: buffer,
        })
    }
}
