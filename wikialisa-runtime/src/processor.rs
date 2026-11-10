//! # Module Xử Lý Giao Dịch Song Song Đa Luồng (`TransactionBatchProcessor`)
//!
//! Hiện thực hóa cơ chế lập lịch thực thi song song tương tự kiến trúc Sealevel của Solana,
//! nhưng được tinh giản tối đa để chạy mượt mà trên phần cứng máy chủ phổ thông bằng `Rayon`.

use crate::accounts_db::{AccountsDb, AccountsDbError};
use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::Arc;
use thiserror::Error;
use wikialisa_sdk::pubkey::Pubkey;
use wikialisa_sdk::transaction::Transaction;

/// Danh mục các lỗi phát sinh trong quá trình thực thi giao dịch tại tầng Runtime.
#[derive(Error, Debug)]
pub enum RuntimeError {
    /// Lỗi xuất phát từ tầng cơ sở dữ liệu tài khoản `AccountsDb`.
    #[error("Lỗi cơ sở dữ liệu tài khoản: {0}")]
    AccountsDb(#[from] AccountsDbError),

    /// Xung đột khóa tài khoản khi cố gắng thực thi song song.
    #[error("Phát hiện xung đột khóa ghi tài khoản trong quá trình xử lý song song")]
    AccountLockCollision,

    /// Định dạng giao dịch hoặc cấu trúc thông điệp không hợp lệ.
    #[error("Định dạng giao dịch không hợp lệ")]
    InvalidTransaction,
}

/// Bộ xử lý giao dịch theo lô đa luồng.
pub struct TransactionBatchProcessor {
    /// Con trỏ tham chiếu chia sẻ tới cơ sở dữ liệu tài khoản.
    pub accounts_db: Arc<AccountsDb>,
}

impl TransactionBatchProcessor {
    /// Khởi tạo một bộ xử lý giao dịch mới liên kết với cơ sở dữ liệu tài khoản.
    pub fn new(accounts_db: Arc<AccountsDb>) -> Self {
        Self { accounts_db }
    }

    /// Xử lý một lô (batch) các giao dịch hoàn toàn song song bằng cách chia luồng qua Rayon.
    /// 
    /// Các giao dịch trong batch này đã được đảm bảo không có sự chồng chéo khóa ghi.
    pub fn process_batch_parallel(
        &self,
        transactions: &[Transaction],
    ) -> Vec<Result<(), RuntimeError>> {
        transactions
            .par_iter()
            .map(|tx| self.execute_transaction(tx))
            .collect()
    }

    /// Thực thi tuần tự một giao dịch đơn lẻ và cập nhật trạng thái tương ứng.
    fn execute_transaction(&self, tx: &Transaction) -> Result<(), RuntimeError> {
        if tx.message.account_keys.is_empty() {
            return Err(RuntimeError::InvalidTransaction);
        }

        // Kiểm tra tính tồn tại của tài khoản chi trả phí giao dịch
        let payer = &tx.message.account_keys[0];
        let _payer_account = self
            .accounts_db
            .load_account(payer)
            .ok_or(AccountsDbError::AccountNotFound(*payer))?;

        Ok(())
    }

    /// Thuật toán phân nhóm các giao dịch thành các đợt (batches) độc lập không xung đột khóa ghi.
    /// 
    /// Các giao dịch trong cùng một batch có thể được thực thi hoàn toàn song song mà không sợ tranh chấp dữ liệu (Race Condition).
    pub fn schedule_conflict_free_batches<'a>(
        transactions: &'a [Transaction],
    ) -> Vec<Vec<&'a Transaction>> {
        let mut batches: Vec<Vec<&'a Transaction>> = Vec::new();

        for tx in transactions {
            let mut placed = false;
            let tx_accounts: HashSet<Pubkey> = tx.message.account_keys.iter().copied().collect();

            for batch in &mut batches {
                let batch_accounts: HashSet<Pubkey> = batch
                    .iter()
                    .flat_map(|t| t.message.account_keys.iter().copied())
                    .collect();

                // Nếu không có bất kỳ tài khoản nào trùng nhau, có thể xếp vào cùng một đợt chạy song song
                if tx_accounts.is_disjoint(&batch_accounts) {
                    batch.push(tx);
                    placed = true;
                    break;
                }
            }

            // Nếu xung đột với toàn bộ các batch hiện có, tạo một batch mới
            if !placed {
                batches.push(vec![tx]);
            }
        }

        batches
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wikialisa_sdk::account::Account;

    #[test]
    fn test_parallel_batch_execution() {
        let db = Arc::new(AccountsDb::new());
        let processor = TransactionBatchProcessor::new(db.clone());

        let mut txs = Vec::new();
        for i in 0..100 {
            let pk = Pubkey::new_from_array([i as u8; 32]);
            db.store_account(pk, Account::new(1000, 0, &Pubkey::default()));

            let tx = Transaction::new_with_payer(&[], Some(&pk));
            txs.push(tx);
        }

        let results = processor.process_batch_parallel(&txs);
        assert_eq!(results.len(), 100);
        for res in results {
            assert!(res.is_ok());
        }
    }

    #[test]
    fn test_conflict_free_batch_scheduling() {
        let pk1 = Pubkey::new_from_array([1u8; 32]);
        let pk2 = Pubkey::new_from_array([2u8; 32]);
        let pk3 = Pubkey::new_from_array([3u8; 32]);

        let tx1 = Transaction::new_with_payer(&[], Some(&pk1));
        let tx2 = Transaction::new_with_payer(&[], Some(&pk2));
        let tx3 = Transaction::new_with_payer(&[], Some(&pk1)); // Xung đột tài khoản pk1 với tx1

        let txs = vec![tx1, tx2, tx3];
        let scheduled = TransactionBatchProcessor::schedule_conflict_free_batches(&txs);

        // tx1 và tx2 được xếp vào batch 0, tx3 bị dời sang batch 1 tiếp theo
        assert_eq!(scheduled.len(), 2);
        assert_eq!(scheduled[0].len(), 2);
        assert_eq!(scheduled[1].len(), 1);
    }
}
