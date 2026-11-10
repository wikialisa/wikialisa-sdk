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

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_pubkey_operations() {
        let pk = Pubkey::new_from_array([42u8; 32]);
        assert_eq!(pk.to_bytes()[0], 42);
        assert_eq!(pk.as_ref().len(), 32);
        let pk_str = format!("{}", pk);
        assert!(!pk_str.is_empty());
    }

    #[test]
    fn test_instruction_and_account_meta() {
        let program_id = Pubkey::new_from_array([1u8; 32]);
        let user_pk = Pubkey::new_from_array([2u8; 32]);
        let meta_signer = AccountMeta::new(user_pk, true);
        let meta_readonly = AccountMeta::new_readonly(program_id, false);

        assert!(meta_signer.is_signer);
        assert!(meta_signer.is_writable);
        assert!(!meta_readonly.is_signer);
        assert!(!meta_readonly.is_writable);

        let ix = Instruction::new_with_borsh(
            program_id,
            &vec![10u8, 20, 30],
            vec![meta_signer, meta_readonly],
        ).expect("Mã hóa instruction Borsh thành công");

        assert_eq!(ix.program_id, program_id);
        assert_eq!(ix.accounts.len(), 2);
    }

    #[test]
    fn test_account_info_creation() {
        let key = Pubkey::new_from_array([3u8; 32]);
        let owner = Pubkey::new_from_array([4u8; 32]);
        let mut lamports_val = 500u64;
        let mut data_buf = vec![0u8; 64];

        let account = AccountInfo::new(
            &key,
            true,
            true,
            Rc::new(RefCell::new(&mut lamports_val)),
            Rc::new(RefCell::new(&mut data_buf)),
            &owner,
            false,
        );

        assert_eq!(*account.key, key);
        assert_eq!(*account.owner, owner);
        assert!(account.is_signer);
        assert!(account.is_writable);
    }

    #[test]
    fn test_program_errors() {
        let err = ProgramError::InsufficientFunds;
        let err_msg = format!("{}", err);
        assert!(err_msg.contains("Số dư tài khoản không đủ"));
    }
}
