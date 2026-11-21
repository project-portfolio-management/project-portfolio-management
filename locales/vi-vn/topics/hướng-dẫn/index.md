# Hướng dẫn

*Các bước chạy bản trình diễn trên máy tính cục bộ và xem một danh mục mẫu.*

Bản trình diễn là một bản sao đầy đủ của phần mềm, được nạp dữ liệu của một tổ chức hư cấu, Northgate Transport Authority. Mọi con người, kế hoạch, con số và ngày tháng đều là hư cấu, và có thể thay đổi dữ liệu tùy ý.

**Lưu ý bảo mật.** Bản trình diễn tin cậy một khóa kiểm thử đã được công bố. Không chạy nó trên mạng dùng chung và không dùng với dữ liệu thật.

## Yêu cầu

- [Git](https://git-scm.com/)
- [Podman](https://podman.io/) với Compose

## Bước 1: tải mã nguồn

```sh
git clone https://github.com/project-portfolio-management/project-portfolio-management.git
cd project-portfolio-management
```

## Bước 2: khởi động bản trình diễn

```sh
POSTGRES_PASSWORD=demo podman compose --profile demo up -d --build --wait ui-demo
```

Lệnh này xây dựng phần mềm, khởi động cơ sở dữ liệu, nạp danh mục hư cấu và khởi động giao diện. Lần xây dựng đầu tiên mất nhiều thời gian hơn các lần sau.

## Bước 3: đăng nhập

Mở <http://localhost:3000/vi-vn/signin>, giữ nguyên địa chỉ `example@example.com` và nhấn **Sign in to the demonstration**. Không có email nào được gửi đi.

Trang đầu tiên là **chuyến tham quan bản trình diễn**, liệt kê mọi tính năng theo nhóm, kèm mô tả và liên kết. Một biểu ngữ màu vàng trên mọi trang cho biết dữ liệu là hư cấu.

Giao diện của bản trình diễn vẫn dùng nhãn tiếng Anh, nên hướng dẫn này giữ nguyên các nhãn đó (Sign in to the demonstration, Plans, Portfolio dashboard, North Line interlocking, Gantt, Critical path, Earned value, My work, My kanban board, Capacity heatmap, Start queue, Director packs).

## Bước 4: xem danh mục

Từ chuyến tham quan, mở **Plans** để xem danh mục, các chương trình và các dự án của nó. Sau đó mở **Portfolio dashboard**.

## Bước 5: xem một dự án

Mở dự án **North Line interlocking** qua nhóm triển khai của chuyến tham quan. Xem biểu đồ **Gantt**, **Critical path** và **Earned value** của dự án.

## Bước 6: xem công việc cá nhân

Mở **My work** và **My kanban board**. Phiên làm việc đăng nhập với tên Example Person, người có các nhiệm vụ đang mở.

## Bước 7: xem năng lực, kiểm soát khởi động và báo cáo

Mở **Capacity heatmap** để xem các nguồn lực bị đặt quá tải. Mở **Start queue**, nơi giới hạn công việc đang thực hiện đã giữ lại một chương trình. Mở **Director packs**, bản tóm tắt hằng tháng được đóng băng sau khi phát hành.

## Bước 8: dừng bản trình diễn

Lệnh này dừng mọi dịch vụ và xóa dữ liệu của bản trình diễn:

```sh
podman compose --profile demo down -v
```

## Các bước tiếp theo

- [Hướng dẫn cài đặt](../../../../INSTALL.md), bao gồm đăng nhập và các nhà cung cấp danh tính
- [Bản tóm tắt theo vai trò](../../for/) cho các vai trò cấp cao
- [Hướng dẫn về bản trình diễn](../../../../operations/demo.md)
- [Danh sách tính năng](../tính-năng/index.md)

Có thể báo cáo sự cố qua [trình theo dõi vấn đề](https://github.com/project-portfolio-management/project-portfolio-management/issues).
