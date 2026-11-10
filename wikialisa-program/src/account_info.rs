//! # Module Thông Tin Tài Khoản (`AccountInfo`)
//!
//! Đại diện cho ngữ cảnh tài khoản khi được nạp vào máy ảo on-chain của Wikialisa.
//! Chứa thông tin về số dư đơn vị tính (lamports), con trỏ vùng nhớ dữ liệu trạng thái,
//! chủ sở hữu tài khoản (owner) và các cờ phân quyền (chữ ký, ghi dữ liệu).

use crate::pubkey::Pubkey;
use std::cell::RefCell;
use std::rc::Rc;

/// Cấu trúc tham chiếu thông tin tài khoản được truyền vào hàm entrypoint của smart contract.
#[derive(Clone, Debug)]
pub struct AccountInfo<'a> {
    /// Khóa công khai định danh tài khoản này.
    pub key: &'a Pubkey,
    /// Số dư đơn vị lamports có thể biến đổi (mutable) qua con trỏ RefCell.
    pub lamports: Rc<RefCell<&'a mut u64>>,
    /// Dữ liệu byte nhị phân lưu trữ trạng thái của tài khoản.
    pub data: Rc<RefCell<&'a mut [u8]>>,
    /// Khóa công khai của chương trình sở hữu và có quyền biến đổi dữ liệu tài khoản này.
    pub owner: &'a Pubkey,
    /// Cờ đánh dấu tài khoản này có phải là mã máy thực thi (executable program) hay không.
    pub executable: bool,
    /// Cờ xác nhận giao dịch hiện tại có chữ ký mật mã hợp lệ của tài khoản này hay không.
    pub is_signer: bool,
    /// Cờ cho phép giao dịch có quyền ghi/sửa đổi số dư hoặc dữ liệu của tài khoản này hay không.
    pub is_writable: bool,
}

impl<'a> AccountInfo<'a> {
    /// Tạo mới một cấu trúc `AccountInfo` với đầy đủ các thuộc tính tham chiếu.
    pub fn new(
        key: &'a Pubkey,
        is_signer: bool,
        is_writable: bool,
        lamports: Rc<RefCell<&'a mut u64>>,
        data: Rc<RefCell<&'a mut [u8]>>,
        owner: &'a Pubkey,
        executable: bool,
    ) -> Self {
        Self {
            key,
            is_signer,
            is_writable,
            lamports,
            data,
            owner,
            executable,
        }
    }
}
