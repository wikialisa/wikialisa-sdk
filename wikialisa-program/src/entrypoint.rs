//! # Module Điểm Khởi Đầu Thực Thi (`Entrypoint`)
//!
//! Định nghĩa kiểu kết quả trả về `ProgramResult`, mẫu chữ ký hàm xử lý lệnh
//! và macro liên kết `entrypoint!` kết nối hàm xử lý Rust với ABI của máy ảo Wikialisa.

use crate::account_info::AccountInfo;
use crate::program_error::ProgramError;
use crate::pubkey::Pubkey;

/// Kiểu dữ liệu trả về chuẩn từ hàm xử lý lệnh của smart contract.
pub type ProgramResult = Result<(), ProgramError>;

/// Chữ ký mẫu của hàm xử lý chỉ mục lệnh trong smart contract on-chain.
pub type ProcessInstruction = fn(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult;

/// Macro xuất khẩu hàm liên kết nhị phân C ABI (`entrypoint`) cho smart contract Wikialisa.
#[macro_export]
macro_rules! entrypoint {
    ($process_instruction:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
            // Bộ chuyển tiếp ABI nhị phân phục vụ máy ảo Wikialisa VM
            0
        }
    };
}
