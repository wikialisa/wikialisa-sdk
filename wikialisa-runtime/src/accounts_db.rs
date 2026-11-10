//! # Module Quản Lý Trạng Thái Tài Khoản Đồng Thời (`AccountsDb`)
//!
//! Cung cấp kho lưu trữ tài khoản phi tập trung trong bộ nhớ (In-memory Concurrent Database),
//! dựa trên cấu trúc bảng băm phân mảnh `DashMap` đa luồng, hỗ trợ đọc và ghi không phong tỏa.

use dashmap::DashMap;
use std::sync::Arc;
use thiserror::Error;
use wikialisa_sdk::account::Account;
use wikialisa_sdk::pubkey::Pubkey;

/// Danh mục mã lỗi phát sinh khi thao tác trên cơ sở dữ liệu tài khoản.
#[derive(Error, Debug)]
pub enum AccountsDbError {
    /// Không tìm thấy tài khoản với khóa công khai tương ứng.
    #[error("Không tìm thấy tài khoản: {0}")]
    AccountNotFound(Pubkey),

    /// Số dư lamports của tài khoản không đủ để thực hiện lệnh trừ tiền.
    #[error("Số dư lamports trong tài khoản không đủ")]
    InsufficientFunds,

    /// Tràn số nguyên trong quá trình cộng/trừ số dư lamports.
    #[error("Lỗi tràn số nguyên khi cập nhật số dư")]
    ArithmeticOverflow,
}

/// Cơ sở dữ liệu trạng thái tài khoản đồng thời, an toàn khi truy xuất từ nhiều luồng.
#[derive(Clone, Default)]
pub struct AccountsDb {
    storage: Arc<DashMap<Pubkey, Account>>,
}

impl AccountsDb {
    /// Khởi tạo một cơ sở dữ liệu tài khoản rỗng.
    pub fn new() -> Self {
        Self {
            storage: Arc::new(DashMap::new()),
        }
    }

    /// Lưu trữ hoặc cập nhật dữ liệu tài khoản ứng với khóa công khai `pubkey`.
    pub fn store_account(&self, pubkey: Pubkey, account: Account) {
        self.storage.insert(pubkey, account);
    }

    /// Đọc thông tin bản sao của tài khoản dựa trên khóa công khai `pubkey`.
    pub fn load_account(&self, pubkey: &Pubkey) -> Option<Account> {
        self.storage.get(pubkey).map(|entry| entry.value().clone())
    }

    /// Thực hiện chuyển khoản số dư lamports giữa hai tài khoản một cách an toàn.
    pub fn transfer(&self, from: &Pubkey, to: &Pubkey, lamports: u64) -> Result<(), AccountsDbError> {
        let mut from_account = self.storage
            .get_mut(from)
            .ok_or(AccountsDbError::AccountNotFound(*from))?;
        
        if from_account.lamports < lamports {
            return Err(AccountsDbError::InsufficientFunds);
        }

        let mut to_account = self.storage
            .entry(*to)
            .or_insert_with(|| Account::new(0, 0, &Pubkey::default()));

        from_account.lamports = from_account
            .lamports
            .checked_sub(lamports)
            .ok_or(AccountsDbError::ArithmeticOverflow)?;

        to_account.lamports = to_account
            .lamports
            .checked_add(lamports)
            .ok_or(AccountsDbError::ArithmeticOverflow)?;

        Ok(())
    }

    /// Trả về tổng số lượng tài khoản hiện có trong cơ sở dữ liệu.
    pub fn count(&self) -> usize {
        self.storage.len()
    }
}
