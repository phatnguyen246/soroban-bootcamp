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
- **Contract ID**: `CD3V47XRHSQKM64TUHKMD6TIJM62UMSEDCAFSKKTACOFO5WUGHBFUYD7`
- **Giao dịch**: https://stellar.expert/explorer/testnet/tx/85a8eb1367cbeb91c440c80c5d2362371222d01174690d893922b765ff6acc4f

## Cách Chạy
1. Clone repo: `git clone https://github.com/phatnguyen246/soroban-bootcamp.git`
2. Vào project: `cd soroban-bootcamp`
3. Build contract: `cd contracts/my-project && stellar contract build`
4. Chạy test contract: `cargo test`
5. Quay về root project: `cd ../..`
6. Chạy frontend bằng static server: `python3 -m http.server 8000`
7. Mở trình duyệt tại: `http://localhost:8000/frontend/`
8. Kết nối Freighter ở mạng Testnet và nhập `Contract ID` để bắt đầu tương tác

> Nếu không có `python3`, bạn có thể chạy frontend bằng `npx serve .` từ thư mục root rồi mở `/frontend/`.

## Tech Stack
- Smart Contract: Rust 2021 / Soroban SDK v22
- Frontend: HTML / CSS / JavaScript thuần trong một file `frontend/index.html`
- Blockchain SDK: `@stellar/stellar-sdk` tải qua CDN
- Wallet Integration: Freighter Wallet API tải qua browser CDN
- Mạng: Stellar Testnet

## Cấu Trúc Repo
- `contracts/my-project/`: smart contract Campus Coin
- `frontend/`: UI HTML/JS để kết nối Freighter và tương tác contract
- `frontend/README.md`: hướng dẫn sử dụng frontend chi tiết

## Nhóm
- Phat Nguyen | 0962856613 | phatnguyenac02@gmail.com | Greenwich + 2026
