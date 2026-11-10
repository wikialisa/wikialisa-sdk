//! # Động Cơ Thực Thi Lõi Wikialisa (`wikialisa-runtime`)
//!
//! Module quản lý trạng thái tài khoản đồng thời và đường ống xử lý giao dịch song song.
//! Được thiết kế nhằm khắc phục các điểm nghẽn hiệu năng của Solana:
//! 1. Loại bỏ hiện tượng nghẽn mạng thông qua cơ chế phân nhóm giao dịch không xung đột khóa ghi.
//! 2. Sử dụng `DashMap` cho cấu trúc dữ liệu tài khoản (`AccountsDb`) giúp truy cập đọc/ghi đồng thời cực nhanh mà không khóa cứng toàn bộ database.
//! 3. Tận dụng thư viện `Rayon` để phân phối các lô giao dịch độc lập chạy song song trên toàn bộ các lõi CPU của máy chủ Validator.

pub mod accounts_db;
pub mod processor;

pub use accounts_db::{AccountsDb, AccountsDbError};
pub use processor::{RuntimeError, TransactionBatchProcessor};
