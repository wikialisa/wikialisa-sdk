//! # Module Mã Lỗi Chương Trình (`ProgramError`)
//!
//! Định nghĩa danh mục các mã lỗi chuẩn hóa trả về từ smart contract on-chain
//! khi gặp sự cố tính toán, thiếu quyền hạn hoặc sai lệch dữ liệu đầu vào.

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Tập hợp các mã lỗi chuẩn của máy ảo smart contract Wikialisa.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum ProgramError {
    /// Lỗi tùy biến do chính logic của smart contract định nghĩa qua mã số u32.
    #[error("Lỗi tùy biến từ smart contract: {0:#x}")]
    Custom(u32),

    /// Tham số đầu vào cung cấp cho chương trình không hợp lệ.
    #[error("Các tham số truyền vào chương trình không hợp lệ")]
    InvalidArgument,

    /// Dữ liệu lệnh (instruction data) bị sai định dạng hoặc không thể giải mã.
    #[error("Dữ liệu của chỉ mục lệnh (instruction data) không hợp lệ")]
    InvalidInstructionData,

    /// Dữ liệu lưu trữ trong tài khoản không tương thích hoặc bị hỏng.
    #[error("Dữ liệu lưu trữ trong tài khoản không hợp lệ")]
    InvalidAccountData,

    /// Vùng nhớ phân bổ cho tài khoản nhỏ hơn kích thước dữ liệu cần lưu.
    #[error("Kích thước vùng nhớ của tài khoản quá nhỏ")]
    AccountDataTooSmall,

    /// Số dư lamports của tài khoản không đủ để thực hiện thao tác trừ tiền.
    #[error("Số dư tài khoản không đủ để hoàn tất giao dịch")]
    InsufficientFunds,

    /// Tài khoản đã được khởi tạo trước đó và không thể khởi tạo lại.
    #[error("Tài khoản này đã được khởi tạo trước đó")]
    AccountAlreadyInitialized,

    /// Tài khoản mục tiêu chưa được khởi tạo trạng thái ban đầu.
    #[error("Tài khoản chưa được khởi tạo")]
    UninitializedAccount,

    /// Thiếu chữ ký mật mã bắt buộc từ một trong các tài khoản yêu cầu quyền ký.
    #[error("Thiếu chữ ký xác thực bắt buộc của tài khoản")]
    MissingRequiredSignature,

    /// Xảy ra xung đột mượn vùng nhớ tài khoản (đang được ghi bởi giao dịch khác).
    #[error("Tài khoản hiện đang được sử dụng hoặc bị khóa bởi giao dịch song song khác")]
    AccountBorrowFailed,

    /// Xảy ra lỗi tràn số nguyên trong quá trình tính toán số dư hoặc phép toán số học.
    #[error("Xảy ra lỗi tràn số nguyên (arithmetic overflow)")]
    ArithmeticOverflow,
}
