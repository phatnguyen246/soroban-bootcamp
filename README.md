# Campus Coin

## Vấn Đề
Sinh viên trong trường đại học gặp khó khăn trong việc thanh toán các khoản nhỏ và trao đổi credit nội bộ một cách minh bạch và nhanh chóng.

## Giải Pháp
Campus Coin là một dApp trên Stellar dùng custom token qua Soroban contract để admin phát hành credit và sinh viên chuyển token cho nhau gần như tức thì với chi phí rất thấp.

## Tại Sao Stellar
Stellar phù hợp vì giao dịch rẻ, xác nhận nhanh và Soroban cho phép triển khai logic token đơn giản, minh bạch trên Testnet.

## Người Dùng Mục Tiêu
Sinh viên đại học, ban quản lý câu lạc bộ, và các nhóm trong trường cần một hệ thống credit nội bộ đơn giản để thanh toán và trao đổi giá trị nhỏ.

## Demo Trực Tiếp
- **Mạng**: Stellar Testnet
- **Contract ID**: `CABC123...`
- **Giao dịch**: https://stellar.expert/explorer/testnet/tx/YOUR_TX_HASH

## Cách Chạy
1. Clone: `git clone https://github.com/yourname/project.git`
2. Build: `cd contracts/my-project && stellar contract build`
3. Test: `cargo test`
4. Deploy: `stellar contract deploy --wasm target/wasm32-unknown-unknown/release/my_project.wasm --source-account student --network testnet`
5. Frontend: `cd frontend && npx serve .`

## Tech Stack
- Smart Contract: Rust / Soroban SDK v22
- Frontend: HTML / JavaScript / `@stellar/stellar-sdk`
- Wallet: Freighter
- Mạng: Stellar Testnet

## Cấu Trúc Repo
- `contracts/my-project/`: smart contract Campus Coin
- `frontend/`: UI HTML/JS để kết nối Freighter và tương tác contract
- `frontend/README.md`: hướng dẫn sử dụng frontend chi tiết

## Nhóm
- [Tên Bạn] | [@telegram] | [email] | [trường + năm]
