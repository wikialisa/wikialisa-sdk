# 🏛️ Wikialisa Rust SDK (`wikialisa-sdk`)
### *Bộ Công Cụ Phát Triển Phần Mềm & Động Cơ Thực Thi Lõi Cho Mạng Lưới Layer 1 Chủ Quyền Wikialisa*

[![Wikialisa Crate](https://img.shields.io/badge/crates.io-wikialisa--sdk-blue.svg)](https://github.com/wikialisa/wikialisa-sdk)
[![Wikialisa Documentation](https://img.shields.io/badge/docs.rs-wikialisa--sdk-green.svg)](https://github.com/wikialisa/wikialisa-sdk)
[![Maintainer: wikialisa-master](https://img.shields.io/badge/Lead%20Architect-wikialisa--master-emerald.svg)](https://github.com/wikialisa)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust Edition: 2021](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org/)
[![Organization: Wikialisa Core Team](https://img.shields.io/badge/Org-Wikialisa%20Foundation-emerald.svg)](https://github.com/wikialisa)

> 👤 **Nhà Phát Triển Hạt Nhân & Người Đóng Góp Hệ Sinh Thái:** **`wikialisa=master`** (`master@wikialisa.org`)  
> *Dẫn dắt kiến trúc Layer 1, thẩm định bảo mật hợp đồng thông minh, tối ưu hóa runtime song song và thiết lập quy trình kiểm thử Web3 5 bước chuẩn quốc tế.*

---

## 📑 Mục Lục Chi Tiết (Table of Contents)

1. [Tổng Quan Dự Án & Sứ Mệnh (Project Overview & Mission)](#1-tổng-quan-dự-án--sứ-mệnh)
2. [Kiến Trúc Workspace Lõi Wikialisa (Core Workspace Architecture)](#2-kiến-trúc-workspace-lõi-wikialisa)
3. [Bản Đồ Bóc Tách Module & Nâng Cấp (Decoupled Module Map & Upgrading)](#3-bản-đồ-bóc-tách-module--nâng-cấp)
4. [Các Thay Đổi Đột Phá Kỹ Thuật (Breaking Changes & Migration Guide)](#4-các-thay-đổi-đột-phá-kỹ-thuật)
5. [Tối Ưu Hóa Phụ Thuộc SPL & Biên Dịch LTO (SPL Interfaces & LTO)](#5-tối-ưu-hóa-phụ-thuộc-spl--biên-dịch-lto)
6. [Hướng Dẫn Cài Đặt & Biên Dịch (Building & Prerequisites)](#6-hướng-dẫn-cài-đặt--biên-dịch)
7. [Dành Cho Nhà Phát Triển Wikialisa Core (For Wikialisa Core Developers)](#7-dành-cho-nhà-phát-triển-wikialisa-core)
8. [Quy Trình Kiểm Thử, Định Dạng & Đo Hiệu Năng (Testing, Linting & Coverage)](#8-quy-trình-kiểm-thử-định-dạng--đo-hiệu-năng)
9. [Bộ Mẫu Mã Nguồn Kèm Chú Thích Tiếng Việt (Code Examples with Vietnamese Docs)](#9-bộ-mẫu-mã-nguồn-kèm-chú-thích-tiếng-việt)
10. [Tuân Thủ Giấy Phép & Tuyên Bố Bản Quyền (License & Legal Compliance)](#10-tuân-thủ-giấy-phép--tuyên-bố-bản-quyền)

---

## 1. TỔNG QUAN DỰ ÁN & SỨ MỆNH

**`wikialisa-sdk`** là bộ công cụ phát triển phần mềm Rust (SDK) và động cơ thực thi nền tảng dành riêng cho mạng lưới blockchain **Wikialisa Layer 1**. 

Dự án được kế thừa và tái cấu trúc hợp pháp từ mã nguồn mở `solana-sdk` (thuộc Solana Foundation & Anza Maintainers), nhằm phục vụ hai mục tiêu chiến lược:
1. **Cung cấp thư viện nền tảng độc lập:** Cho phép các lập trình viên xây dựng smart contract (hợp đồng thông minh) on-chain và các ứng dụng dApp off-chain với hiệu năng xử lý cực đại.
2. **Khắc phục triệt để các điểm nghẽn hệ thống:** Loại bỏ sự phụ thuộc cồng kềnh của cụm validator cũ, tái kiến trúc bộ nhớ trạng thái nhằm chống phình to tài nguyên (State Bloat) và phân phối luồng xử lý song song không khóa cứng (Lock-free Parallel Runtime) bằng **Rayon** và **DashMap**.

---

## 2. KIẾN TRÚC WORKSPACE LÕI WIKIALISA

Workspace tại thư mục gốc được tinh giản và tái cơ cấu thành 4 crate cốt lõi trong tệp `Cargo.toml`:

```text
wikialisa-sdk/
├── Cargo.toml                    # Cấu hình không gian làm việc chính (Workspace Root)
├── Cargo.lock                    # Tệp khóa phiên bản phụ thuộc thống nhất
├── wikialisa-program/            # Thư viện smart contract on-chain chuẩn hóa
│   ├── README.md                 # Hướng dẫn & Giới thiệu wikialisa-master cho Smart Contract
│   ├── Cargo.toml
│   └── src/                      # Primitives: Pubkey, AccountInfo, Instruction, ProgramError
├── wikialisa-sdk/                # SDK trung tâm cho các nhà phát triển ứng dụng dApp
│   ├── README.md                 # Hướng dẫn & Giới thiệu wikialisa-master cho dApp Devs
│   ├── Cargo.toml
│   └── src/                      # Transaction, Signature, Signer, Keypair, Aliases
├── wikialisa-runtime/            # Động cơ thực thi giao dịch song song & quản trị tài khoản
│   ├── README.md                 # Hướng dẫn & Giới thiệu wikialisa-master cho Parallel VM
│   ├── Cargo.toml
│   └── src/                      # AccountsDb (DashMap), TransactionBatchProcessor (Rayon)
└── wikialisa-client/             # Thư viện RPC Client kết nối node Layer 1
    ├── README.md                 # Hướng dẫn & Giới thiệu wikialisa-master cho RPC & Network
    ├── Cargo.toml
    └── src/                      # RpcClient, API truy vấn tài khoản và gửi giao dịch
```

👉 **Tài Liệu Chi Tiết Từng Module Được Quản Lý Bởi `wikialisa-master`:**
- [📜 **`wikialisa-program/README.md`**](wikialisa-program/README.md): Bản đồ kiểm toán, công cụ Fuzzing và mẫu báo cáo lỗ hổng Reentrancy.
- [📦 **`wikialisa-sdk/README.md`**](wikialisa-sdk/README.md): Bộ công cụ xây dựng giao dịch, đa chữ ký và chiến lược chuyển đổi Testnet.
- [⚡ **`wikialisa-runtime/README.md`**](wikialisa-runtime/README.md): Động cơ thực thi song song Rayon, cơ sở dữ liệu DashMap và Docker Node runner.
- [🌐 **`wikialisa-client/README.md`**](wikialisa-client/README.md): RPC Gateway, API truy vấn và mẫu PR khắc phục Memory Leak khi State Sync.

---

## 3. BẢN ĐỒ BÓC TÁCH MODULE & NÂNG CẤP

Để tối ưu hóa thời gian biên dịch và cho phép thực hiện Link-Time Optimization (LTO), toàn bộ các module nguyên khối cũ đã được phân rã thành các crate giao diện chuyên biệt:

### 3.1. Phân Rã Các Module Của `wikialisa-sdk` (Tiền Thân `solana-sdk`)

| Module Cũ (Legacy Module) | Crate Giao Diện Wikialisa Thay Thế | Mục Đích Kỹ Thuật & Mô Tả |
| :--- | :--- | :--- |
| `address_lookup_table` | `solana_address_lookup_table_interface` | Bảng tra cứu địa chỉ nén dung lượng giao dịch |
| `alt_bn128` | `solana_bn254` | Phép toán đường cong elip mật mã học BN254 |
| `bpf_loader_upgradeable` | `solana_loader_v3_interface` | Giao diện nạp và nâng cấp smart contract phiên bản 3 |
| `client` | `solana_client_traits` | Định nghĩa trait giao tiếp RPC độc lập với client |
| `commitment_config` | `solana_commitment_config` | Cấu hình mức độ cam kết và hoàn tất khối (Finality) |
| `compute_budget` | `solana_compute_budget_interface` | Giới hạn đơn vị tính toán (Compute Units) và phí ưu tiên |
| `decode_error` | `solana_decode_error` | Tiện ích giải mã mã lỗi smart contract |
| `derivation_path` | `solana_derivation_path` | Đường dẫn dẫn xuất khóa phân tầng BIP-32/BIP-44 |
| `ed25519_instruction` | `solana_ed25519_program` | Lệnh tiền biên dịch xác thực chữ ký Ed25519 |
| `exit` | `solana_validator_exit` | Tín hiệu điều khiển dừng node an toàn |
| `feature_set` / `feature` | `solana_feature_gate_interface` | Cơ chế kích hoạt tính năng mạng qua biểu quyết |
| `genesis_config` | `solana_genesis_config` | Cấu hình tham số khối nguyên thủy (Genesis Block) |
| `hard_forks` | `solana_hard_forks` | Quản trị danh sách các điểm phân kỳ cứng của mạng |
| `loader_instruction` | `solana_loader_v2_interface` | Giao diện nạp chương trình phiên bản 2 kế thừa |
| `loader_upgradeable_instruction` | `solana_loader_v3_interface::instruction` | Tập lệnh quản lý chương trình có thể nâng cấp |
| `nonce` / `nonce_account` | `solana_nonce` / `solana_nonce_account` | Quản lý giao dịch bất đồng bộ qua cơ chế Durable Nonce |
| `packet` | `solana_packet` | Cấu trúc gói tin mạng UDP/QUIC thô |
| `poh_config` | `solana_poh_config` | Tham số tần số xung nhịp đồng hồ Proof-of-History |
| `precompiles` | `agave_precompiles` | Tập hợp các lệnh tính toán mã hóa tối ưu hóa cao |
| `program_utils` | `solana_bincode::limited_deserialize` | Giải mã nhị phân có giới hạn bộ đệm chống tràn RAM |
| `quic` | `solana_quic_definitions` | Định nghĩa giao thức truyền tải khối bảo mật QUIC |
| `reward_info` / `reward_type` | `solana_reward_info` | Cấu trúc thông tin phân bổ phần thưởng Staking |
| `secp256k1_instruction` | `solana_secp256k1_program` | Xác thực chữ ký chuẩn Ethereum/Bitcoin qua precompile |
| `secp256k1_recover` | `solana_secp256k1_recover` | Khôi phục khóa công khai từ chữ ký ECDSA |
| `stake` / `stake_history` | `solana_stake_interface` | Giao thức Staking, ủy quyền và lịch sử kỷ nguyên (Epoch) |
| `system_instruction` / `system_program` | `solana_system_interface` | Giao diện Chương trình Hệ thống (tạo tài khoản, chuyển tiền) |
| `system_transaction` | `solana_system_transaction` | Tiện ích xây dựng nhanh các giao dịch hệ thống |
| `transaction_context` | `solana_transaction_context` | Ngữ cảnh thực thi chỉ mục tài khoản trong VM |
| `vote` | `solana_vote_interface` | Giao thức bỏ phiếu đồng thuận của Validator |

---

## 4. CÁC THAY ĐỔI ĐỘT PHÁ KỸ THUẬT (BREAKING CHANGES)

Khi tích hợp hoặc nâng cấp mã nguồn, các lập trình viên cần lưu ý các thay đổi đột phá sau:

### 4.1. Kiểu Dữ Liệu `Address` và `Pubkey`
- Giới thiệu kiểu dữ liệu `Address` linh hoạt và ngữ nghĩa rõ ràng hơn để thay thế `Pubkey`.
- `Pubkey` hiện tại được định nghĩa như một bí danh kiểu (Type Alias) của `Address`. 
- Trong mã nguồn của Wikialisa, cả hai kiểu đều hỗ trợ chuyển đổi không mất mát (zero-cost abstraction) và hoàn toàn tương thích ngược.

### 4.2. Quản Trị Trạng Thái Trong `AccountInfo`
- Loại bỏ trường `rent_epoch` cũ (đổi tên thành `_unused`), trường này hoàn toàn có thể bỏ qua trong logic tính toán.
- Tham số cuối cùng trong hàm khởi tạo `AccountInfo::new()` đã được loại bỏ để tương thích với cơ chế lưu trữ tối giản của Wikialisa.

### 4.3. Đóng Gói Cấu Trúc Băm `Hash`
- Các byte nội bộ của cấu trúc `Hash` đã được chuyển sang chế độ riêng tư (`private`).
- Lập trình viên bắt buộc phải sử dụng phương thức `Hash::as_bytes()` hoặc `Hash::to_bytes()` để truy xuất mảng byte thô.

### 4.4. Xử Lý Lỗi `ProgramError` & `InstructionError` Tối Ưu Bộ Nhớ
- Thay đổi `BorshIoError(String)` thành `BorshIoError` (loại bỏ chuỗi `String` phân bổ động trên heap).
- Giúp giảm dung lượng mã máy nhị phân của smart contract, tiết kiệm chi phí đơn vị tính toán (Compute Units).

### 4.5. An Toàn Bộ Nhớ Trong On-chain Program Memory
- Toàn bộ các hàm thao tác bộ nhớ trực tiếp (`solana_program::program_memory`) được đánh dấu là `unsafe`.
- Mọi lời gọi thao tác sao chép con trỏ thô bắt buộc phải nằm trong khối `unsafe { ... }` để đảm bảo tính toàn vẹn bộ nhớ của Wikialisa OS.

---

## 5. TỐI ƯU HÓA PHỤ THUỘ SPL & BIÊN DỊCH LTO

Để hỗ trợ tính năng **Link-Time Optimization (LTO)** giúp giảm thiểu kích thước binary của smart contract, hệ sinh thái thư viện đã được tách đôi thành hai dạng crate:
1. **Interface Crates (`*-interface`):** Chỉ chứa định nghĩa cấu trúc dữ liệu, chỉ mục lệnh và mã lỗi (chỉ khai báo mục tiêu thư viện `lib`). Thích hợp cho việc nhúng vào dApp và smart contract khác với thời gian biên dịch cực nhanh.
2. **Program Crates (`*-program`):** Chứa mã máy thực thi hoàn chỉnh (khai báo mục tiêu `cdylib`).

### Bảng Chuyển Đổi Sang Crate Giao Diện:
- `spl-token` $\rightarrow$ chuyển sang sử dụng `spl-token-interface`
- `spl-token-2022` $\rightarrow$ chuyển sang sử dụng `spl-token-2022-interface`
- `spl-associated-token-account` $\rightarrow$ chuyển sang sử dụng `spl-associated-token-account-interface`
- `spl-memo` $\rightarrow$ chuyển sang sử dụng `spl-memo-interface`

> 💡 **Mẹo biên dịch:** Sử dụng cờ `--lto` khi build smart contract:
> ```bash
> cargo build-sbf --lto
> ```

---

## 6. HƯỚNG DẪN CÀI ĐẶT & BIÊN DỊCH

### 6.1. Cài Đặt Môi Trường Rust & Toolchain Chuẩn
Dự án yêu cầu phiên bản Rust ổn định (hoặc bản nightly hỗ trợ platform-tools). Cài đặt công cụ qua `rustup`:

```bash
# 1. Cài đặt bộ công cụ Rustup
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env

# 2. Cài đặt các thành phần định dạng mã nguồn
rustup component add rustfmt clippy
```

### 6.2. Clone Mã Nguồn Từ Kho Lưu Trữ Wikialisa Chính Thức
```bash
# Clone kho lưu trữ Wikialisa SDK
git clone https://github.com/wikialisa/wikialisa-sdk.git
cd wikialisa-sdk

# Kiểm tra phiên bản toolchain được ghim trong rust-toolchain.toml
rustup show
```

### 6.3. Biên Dịch & Chạy Kiểm Thử Toàn Bộ Workspace
```bash
# Biên dịch toàn bộ các crate trong workspace
cargo build --workspace

# Chạy toàn bộ test suite
cargo test --workspace
```

---

## 7. DÀNH CHO NHÀ PHÁT TRIỂN WIKIALISA CORE

### 7.1. Vá Cục Bộ Các Crate Khi Phát Triển Cùng Wikialisa Node / Agave
Nếu quá trình phát triển node mạng yêu cầu cập nhật đồng thời SDK, bạn có thể áp dụng script liên kết cục bộ:

```bash
# Vá toàn bộ các crate sang thư mục mã nguồn node
./scripts/patch-crates-no-header.sh <NODE_PATH> <WIKIALISA_SDK_PATH>

# Ví dụ vá riêng lẻ module bn254
./scripts/patch-crates-no-header.sh ../wikialisa-core . bn254
```

### 7.2. Quy Trình Xuất Bản Crate (Publishing Workflows)
Các crate trong `wikialisa-sdk` được quản lý phiên bản độc lập:
1. Kích hoạt GitHub Action **"Publish Crate"** trên giao diện kho lưu trữ.
2. Nhập đường dẫn thư mục crate cần phát hành (ví dụ: `wikialisa-sdk` hoặc `wikialisa-runtime`).
3. Chọn mức độ nâng phiên bản: `patch`, `minor`, `major` hoặc chuỗi phiên bản tường minh.
4. Hệ thống CI/CD sẽ tự động chạy kiểm thử, tạo tag Git, phát hành lên registry và tạo GitHub Release.

### 7.3. Chính Sách Backport Bản Vá Bảo Mật
Khi muốn đưa một bản sửa lỗi về các nhánh bảo trì dài hạn:
- Gắn nhãn `backport <TÊN_NHÁNH>` vào Pull Request (ví dụ: `backport maintenance/v2.x`).
- Bot tự động sẽ mở PR cherry-pick tương ứng vào nhánh đích.

---

## 8. QUY TRÌNH KIỂM THỬ, ĐỊNH DẠNG & ĐO HIỆU NĂNG

### 8.1. Kiểm Thử Cơ Bản & Nâng Cao
```bash
# Chạy toàn bộ unit test
cargo test

# Chạy kiểm thử với script tự động
./scripts/test-stable.sh
```

### 8.2. Định Dạng Mã Nguồn (Rustfmt)
```bash
# Tự động định dạng mã nguồn toàn repo bằng nightly rustfmt
./cargo nightly fmt --all

# Kiểm tra quy chuẩn định dạng (CI check)
./scripts/check-fmt.sh
```

### 8.3. Quét Lỗi Tĩnh (Clippy Linting)
```bash
# Quét cảnh báo tiềm ẩn và chuẩn hóa mã nguồn
./scripts/check-clippy.sh
```

### 8.4. Đo Lường Hiệu Năng (Benchmarking)
```bash
# Khởi chạy các bài đo lường tốc độ xử lý giao dịch
./scripts/test-bench.sh
```

### 8.5. Đo Độ Bao Phủ Mã Nguồn (Code Coverage)
Yêu cầu cài đặt thành phần `llvm-tools-preview`:
```bash
# Cài đặt công cụ đo lường
rustup component add llvm-tools-preview

# Sinh báo cáo độ bao phủ mã nguồn dạng HTML trực quan
./scripts/test-coverage.sh
open target/cov/lcov-local/index.html
```

---

## 9. BỘ MẪU MÃ NGUỒN KÈM CHÚ THÍCH TIẾNG VIỆT

Dưới đây là các ví dụ minh họa cách lập trình ứng dụng trên mạng lưới Wikialisa với đầy đủ chú thích tài liệu bằng tiếng Việt.

### 9.1. Viết Smart Contract On-Chain Với `wikialisa-program`

```rust
//! Module smart contract mẫu trên mạng lưới Wikialisa Layer 1.
//! Thực hiện ghi nhận thông điệp và xác thực tài khoản ký hợp lệ.

use wikialisa_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Khai báo hàm entrypoint chính thức cho smart contract
entrypoint!(process_instruction);

/// Hàm xử lý lệnh thực thi chính của smart contract.
/// 
/// # Tham số:
/// - `program_id`: Khóa công khai định danh của chính chương trình này trên mạng Wikialisa.
/// - `accounts`: Danh sách các tài khoản tham gia vào giao dịch (đọc/ghi/ký).
/// - `instruction_data`: Mảng byte chứa dữ liệu đầu vào do người dùng truyền vào.
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Ghi log chẩn đoán ra môi trường máy ảo Wikialisa VM
    msg!("Wikialisa Smart Contract đang thực thi với Program ID: {:?}", program_id);

    // Duyệt danh sách tài khoản theo thứ tự truyền vào
    let accounts_iter = &mut accounts.iter();
    let signer_account = next_account_info(accounts_iter)?;

    // Kiểm tra tính hợp lệ: Bắt buộc tài khoản người gửi phải có chữ ký hợp lệ
    if !signer_account.is_signer {
        msg!("Lỗi xác thực: Tài khoản người gửi bắt buộc phải ký vào giao dịch!");
        return Err(ProgramError::MissingRequiredSignature);
    }

    msg!("Xác thực chữ ký thành công cho tài khoản: {:?}", signer_account.key);
    msg!("Độ dài dữ liệu lệnh nhận được: {} bytes", instruction_data.len());

    // Hoàn tất thực thi thành công
    Ok(())
}
```

---

### 9.2. Khởi Tạo, Ký & Gửi Giao Dịch Bằng `wikialisa-sdk`

```rust
//! Ứng dụng dApp client mẫu: Khởi tạo cặp khóa ví, đóng gói giao dịch và ký duyệt.

use wikialisa_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signer::{Keypair, Signer},
    transaction::Transaction,
};

fn main() {
    println!("--- BẮT ĐẦU QUY TRÌNH GIAO DỊCH TRÊN WIKIALISA LAYER 1 ---");

    // 1. Tạo ngẫu nhiên một cặp khóa ví mới cho người gửi tiền
    let sender_wallet = Keypair::new();
    println!("Địa chỉ ví người gửi: {}", sender_wallet.pubkey());

    // 2. Định nghĩa địa chỉ ví người nhận tiền
    let receiver_pubkey = Pubkey::new_from_array([5u8; 32]);
    println!("Địa chỉ ví người nhận: {}", receiver_pubkey);

    // 3. Định danh chương trình mục tiêu cần tương tác (ví dụ Chương trình Chuyển khoản)
    let system_program_id = Pubkey::new_from_array([0u8; 32]);

    // 4. Thiết lập danh sách các tài khoản liên quan kèm quyền hạn
    let account_metas = vec![
        AccountMeta::new(sender_wallet.pubkey(), true),        // Tài khoản ghi & có chữ ký
        AccountMeta::new_readonly(receiver_pubkey, false),    // Tài khoản chỉ đọc
    ];

    // 5. Đóng gói chỉ mục lệnh (Instruction)
    let transfer_instruction = Instruction {
        program_id: system_program_id,
        accounts: account_metas,
        data: vec![1, 0, 0, 0, 100, 0, 0, 0], // Dữ liệu mã hóa: Lệnh chuyển 100 lamports
    };

    // 6. Xây dựng giao dịch hoàn chỉnh với người chi trả phí là ví người gửi
    let mut tx = Transaction::new_with_payer(
        &[transfer_instruction],
        Some(&sender_wallet.pubkey()),
    );

    // 7. Lấy mã băm khối gần nhất (Recent Blockhash) để chống tấn công phát lại (Replay Attack)
    let mock_recent_blockhash = [9u8; 32];

    // 8. Ký duyệt giao dịch bằng khóa bí mật của người gửi
    tx.sign(&[&sender_wallet], mock_recent_blockhash);

    println!("Giao dịch đã được ký thành công! Số lượng chữ ký: {}", tx.signatures.len());
    println!("Chi tiết giao dịch: {:?}", tx);
}
```

---

### 9.3. Xử Lý Giao Dịch Song Song Bằng `wikialisa-runtime`

```rust
//! Minh họa động cơ thực thi song song đa luồng (Multi-threaded Rayon Processor)
//! Tự động lập lịch các giao dịch không xung đột khóa ghi để chạy đồng thời.

use std::sync::Arc;
use wikialisa_runtime::{AccountsDb, TransactionBatchProcessor};
use wikialisa_sdk::{
    account::Account,
    pubkey::Pubkey,
    transaction::Transaction,
};

fn main() {
    println!("--- KHỞI TẠO ĐỘNG CƠ RUNTIME SONG SONG WIKIALISA ---");

    // 1. Khởi tạo cơ sở dữ liệu trạng thái tài khoản không khóa cứng (DashMap)
    let accounts_db = Arc::new(AccountsDb::new());

    // 2. Nạp dữ liệu tài khoản thử nghiệm vào hệ thống
    let pubkey_a = Pubkey::new_from_array([1u8; 32]);
    let pubkey_b = Pubkey::new_from_array([2u8; 32]);
    let pubkey_c = Pubkey::new_from_array([3u8; 32]);

    accounts_db.store_account(pubkey_a, Account::new(5_000_000, 0, &Pubkey::default()));
    accounts_db.store_account(pubkey_b, Account::new(8_000_000, 0, &Pubkey::default()));
    accounts_db.store_account(pubkey_c, Account::new(2_000_000, 0, &Pubkey::default()));

    // 3. Khởi tạo bộ xử lý giao dịch theo lô (Batch Processor)
    let processor = TransactionBatchProcessor::new(accounts_db.clone());

    // 4. Tạo danh sách các giao dịch độc lập
    let tx1 = Transaction::new_with_payer(&[], Some(&pubkey_a));
    let tx2 = Transaction::new_with_payer(&[], Some(&pubkey_b));
    let tx3 = Transaction::new_with_payer(&[], Some(&pubkey_c));
    let tx_batch = vec![tx1, tx2, tx3];

    // 5. Tự động phân chia nhóm giao dịch không xung đột
    let conflict_free_batches = TransactionBatchProcessor::schedule_conflict_free_batches(&tx_batch);
    println!("Số lượng đợt giao dịch song song được lên lịch: {}", conflict_free_batches.len());

    // 6. Thực thi song song toàn bộ giao dịch trên các luồng CPU qua Rayon
    let execution_results = processor.process_batch_parallel(&tx_batch);
    for (idx, result) in execution_results.iter().enumerate() {
        match result {
            Ok(_) => println!("Giao dịch #{}: Thực thi thành công!", idx + 1),
            Err(e) => println!("Giao dịch #{}: Thất bại với lỗi: {:?}", idx + 1, e),
        }
    }
}
```

---

## 10. TUÂN THỦ GIẤY PHÉP & TUYÊN BỐ BẢN QUYỀN

### 10.1. Giấy Phép Mã Nguồn Mở (Open Source License)
Dự án được phân phối dưới các điều khoản của **Apache License, Version 2.0** ([LICENSE](LICENSE) hoặc http://www.apache.org/licenses/LICENSE-2.0).

### 10.2. Tuyên Bố Bản Quyền Kế Thừa (Attribution Notice)
- Mã nguồn nền tảng ban đầu được kế thừa từ dự án `solana-sdk`, bản quyền thuộc về **Solana Foundation** và **Anza Maintainers** (`maintainers@anza.xyz`).
- Toàn bộ các cải tiến kiến trúc, phân rã crate, động cơ thực thi song song `wikialisa-runtime`, hệ thống tài liệu tiếng Việt và lộ trình tiến hóa Layer 1 thuộc bản quyền của:  
  **Wikialisa Foundation & Wikialisa Core Team** (`dev@wikialisa.org`).
