# Campus Coin Frontend

Frontend này là một trang `index.html` chạy trực tiếp trong trình duyệt để tương tác với smart contract `MyToken` trên **Stellar Testnet** bằng **Freighter Wallet**.

## Mục tiêu

UI hỗ trợ 3 luồng chính:

- Kết nối ví Freighter
- Khởi tạo token bằng `initialize(admin, name, symbol)`
- Mint và transfer token thật trên Soroban Testnet

Ngoài ra UI cũng có thể:

- Đọc `name()`
- Đọc `symbol()`
- Đọc `total_supply()`
- Đọc `balance(address)` của ví đang kết nối

## Cấu trúc thư mục

```text
frontend/
├── index.html
└── README.md
```

## Yêu cầu trước khi chạy

Bạn nên chuẩn bị sẵn:

- Freighter Wallet đã cài trên trình duyệt
- Freighter đang ở **Testnet**
- Smart contract đã được deploy
- Có `Contract ID`
- Ví testnet có XLM để trả phí giao dịch

## Cách chạy UI

Từ thư mục gốc của repo:

```bash
cd /home/phat/Project/soroban-bootcamp
python3 -m http.server 8000
```

Sau đó mở trình duyệt tại:

```text
http://localhost:8000/frontend/
```

Nếu không có `python3`, có thể dùng:

```bash
cd /home/phat/Project/soroban-bootcamp
npx serve .
```

## Cách sử dụng

### 1. Kết nối ví

- Mở trang frontend
- Bấm `Kết nối Freighter`
- Cho phép site truy cập ví

Sau khi kết nối thành công:

- địa chỉ ví sẽ xuất hiện trên UI
- ô `From wallet` của phần transfer sẽ tự điền

### 2. Nhập Contract ID

Trong phần `Cấu hình contract`:

- dán `Contract ID` đã deploy trên Testnet
- bấm `Lưu cấu hình`
- bấm `Refresh Metadata`

Nếu contract đã được khởi tạo trước đó, UI sẽ cố đọc:

- tên token
- ký hiệu token
- tổng cung
- số dư ví đang kết nối

## Luồng admin

### Khởi tạo token

Form `Admin actions` dùng để gọi:

```text
initialize(admin, name, symbol)
```

Lưu ý:

- contract hiện tại **không nhận tổng cung ban đầu**
- admin phải chính là **ví đang kết nối**
- nếu ô `Admin address` để trống, UI sẽ dùng ví đang kết nối

Các bước:

1. Kết nối ví admin
2. Nhập `Contract ID`
3. Nhập `Token name` và `Token symbol`
4. Bấm `Initialize Token`

Sau khi thành công:

- UI hiển thị transaction hash
- metadata sẽ được refresh lại

### Mint token

Form mint dùng để gọi:

```text
mint(to, amount)
```

Lưu ý:

- chỉ admin mới mint được
- admin phải là ví đang ký bằng Freighter

Các bước:

1. Kết nối ví admin
2. Nhập địa chỉ người nhận ở `Mint to`
3. Nhập số lượng ở `Mint amount`
4. Bấm `Mint Token`

## Luồng sinh viên

Phần `Student transfer` dùng để gọi:

```text
transfer(from, to, amount)
```

Trong UI này:

- `from` luôn là **ví đang kết nối**
- bạn chỉ cần nhập `to` và `amount`

Các bước:

1. Kết nối ví người gửi
2. Nhập `Contract ID`
3. Bấm `Refresh Metadata`
4. Nhập địa chỉ người nhận
5. Nhập số lượng chuyển
6. Có thể bấm `Check Balance` trước
7. Bấm `Transfer Token`

## Transaction result và Activity log

UI có 2 khu vực quan trọng:

- `Transaction result`: hiển thị trạng thái giao dịch mới nhất, hash và link Stellar Expert
- `Activity log`: lưu lại lịch sử các thao tác như connect, refresh, initialize, mint, transfer

## Những giới hạn hiện tại

Frontend này được viết để khớp với contract Rust hiện tại trong repo:

- Có `initialize`
- Có `mint`
- Có `transfer`
- Có `balance`
- Có `name`
- Có `symbol`
- Có `total_supply`

Contract **không có hàm `admin()`**, nên:

- UI không thể đọc admin trực tiếp từ chain
- UI chỉ lưu “admin hiện tại” theo dữ liệu local trong trình duyệt sau khi bạn initialize hoặc cấu hình

Ngoài ra:

- UI đang cố định mạng là **Stellar Testnet**
- UI dùng CDN ESM, không cần bundler
- UI là file HTML thuần, chưa dùng React/Vite

## Lỗi thường gặp

### Không kết nối được Freighter

Kiểm tra:

- Freighter đã cài chưa
- Freighter có đang mở quyền cho website không
- Freighter có đang ở Testnet không

### Refresh metadata thất bại

Có thể do:

- `Contract ID` sai
- contract chưa được initialize
- RPC testnet tạm thời lỗi

### Mint thất bại

Có thể do:

- ví đang kết nối không phải admin
- contract chưa initialize
- amount không hợp lệ

### Transfer thất bại

Có thể do:

- số dư không đủ
- địa chỉ người nhận không hợp lệ
- ví chưa kết nối

## Gợi ý test nhanh

1. Kết nối ví admin
2. Nhập `Contract ID`
3. Initialize token
4. Mint token cho một ví testnet khác
5. Kết nối bằng ví đã nhận token
6. Refresh metadata
7. Transfer token sang ví khác

## Ghi chú

Nếu sau này bạn muốn hỗ trợ:

- tổng cung ban đầu lúc khởi tạo
- đọc admin trực tiếp từ on-chain
- nhiều màn hình hoặc state phức tạp hơn

thì nên mở rộng smart contract trước, sau đó mới nâng cấp frontend.
