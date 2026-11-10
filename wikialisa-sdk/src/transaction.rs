//! # Module Cấu Trúc Giao Dịch (`Transaction`)
//!
//! Định nghĩa cấu trúc thông điệp (Message) và giao dịch hoàn chỉnh (Transaction)
//! chứa danh sách các lệnh thực thi, các tài khoản liên quan và tập hợp chữ ký mật mã.

use crate::signature::Signature;
use crate::signer::Signer;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

/// Thông điệp giao dịch chứa toàn bộ nội dung cần thực thi trên mạng lưới.
#[derive(Clone, Debug, Default, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct Message {
    /// Danh sách toàn bộ các địa chỉ tài khoản tham gia vào giao dịch (đọc/ghi/ký).
    pub account_keys: Vec<Pubkey>,
    /// Mã băm khối gần nhất (Recent Blockhash) dùng để xác định tính hợp lệ thời gian và chống replay attack.
    pub recent_blockhash: [u8; 32],
    /// Danh sách các chỉ mục lệnh thực thi tuần tự trong giao dịch.
    pub instructions: Vec<Instruction>,
}

/// Giao dịch hoàn chỉnh sẵn sàng được phát tán và đưa vào khối của Wikialisa Layer 1.
#[derive(Clone, Debug, Default, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct Transaction {
    /// Danh sách các chữ ký mật mã tương ứng với các tài khoản yêu cầu quyền ký.
    pub signatures: Vec<Signature>,
    /// Nội dung thông điệp giao dịch đã được ký duyệt.
    pub message: Message,
}

impl Transaction {
    /// Khởi tạo một giao dịch chưa ký từ một thông điệp cho trước.
    pub fn new_unsigned(message: Message) -> Self {
        let signatures = vec![Signature::default(); message.account_keys.len()];
        Self {
            signatures,
            message,
        }
    }

    /// Tiện ích xây dựng giao dịch mới từ danh sách các chỉ mục lệnh và tài khoản chi trả phí.
    pub fn new_with_payer(instructions: &[Instruction], payer: Option<&Pubkey>) -> Self {
        let mut account_keys = Vec::new();
        if let Some(payer_pk) = payer {
            account_keys.push(*payer_pk);
        }
        for ix in instructions {
            if !account_keys.contains(&ix.program_id) {
                account_keys.push(ix.program_id);
            }
            for meta in &ix.accounts {
                if !account_keys.contains(&meta.pubkey) {
                    account_keys.push(meta.pubkey);
                }
            }
        }
        let message = Message {
            account_keys,
            recent_blockhash: [0u8; 32],
            instructions: instructions.to_vec(),
        };
        Self::new_unsigned(message)
    }

    /// Ký duyệt giao dịch bằng danh sách các đối tượng triển khai trait `Signer`.
    pub fn sign<T: Signer>(&mut self, signers: &[&T], recent_blockhash: [u8; 32]) {
        self.message.recent_blockhash = recent_blockhash;
        let serialized_message = borsh::to_vec(&self.message).unwrap_or_default();
        self.signatures = signers
            .iter()
            .map(|s| s.sign_message(&serialized_message))
            .collect();
    }
}
