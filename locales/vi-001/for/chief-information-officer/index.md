# Dành cho Giám đốc thông tin

*Giám đốc thông tin (CIO): tích hợp, danh tính, dữ liệu, lưu trữ và các thông tin bạn sẽ được yêu cầu cung cấp.*

## Danh tính và truy cập

- Đăng nhập được **bắt buộc theo mặc định**; dịch vụ từ chối khởi động nếu thiếu khóa xác minh token, và từ chối khởi động ở môi trường vận hành khi đăng nhập bị tắt.
- Đăng nhập qua một nhà cung cấp danh tính (gồm cả Microsoft Entra ID); xác thực đa yếu tố do nhà cung cấp đảm nhiệm ([hướng dẫn đăng nhập](../../../../operations/sign-in-with-entra.md)).
- Việc ủy quyền dùng các thuộc tính của token (ví dụ `access`, `directorpack`, `confidence`) với một chính sách theo thuộc tính. Các kế hoạch riêng tư bị ẩn khỏi mọi số liệu tổng hợp; các dấu xử lý ẩn các kế hoạch trên mức được phép của người gọi, kể cả quản trị viên.
- Truy cập khẩn cấp vào các kế hoạch riêng tư cần đủ điều kiện, lý do nêu rõ, tối đa bốn giờ, và một người thứ hai được báo.
- Quản trị viên có thể liệt kê các quyền truy cập đang có (công cụ hỗ trợ chứng nhận lại) và xuất hoặc ẩn danh hóa bằng bí danh dữ liệu của một người.

## Dữ liệu và tích hợp

| Chủ đề | Hiện trạng |
|---|---|
| Dữ liệu được lưu | Bản ghi kế hoạch, nhiệm vụ, rủi ro, ngân sách, cột mốc, thời gian và chi phí; con người là các tham chiếu ẩn danh, không phải tên. Theo thiết kế, không có dữ liệu bệnh nhân và không có dữ liệu thuộc loại đặc biệt |
| Bản kê | Mọi cột chứa dữ liệu cá nhân đều được liệt kê trong [bản kê dữ liệu cá nhân](../../../../governance/personal-data-inventory.csv); một bài kiểm thử giữ cho bản kê khớp với lược đồ |
| Giao diện | REST với một tài liệu OpenAPI được suy ra; nhập và xuất hàng loạt; nguồn cấp lịch; sự kiện gửi đi và webhook chỉ đến các địa chỉ `https` công khai |
| Sao lưu | Sao lưu bằng kịch bản kèm khôi phục đã được kiểm thử, mã hóa tùy chọn, các mục tiêu khôi phục nêu rõ ([sao lưu và khôi phục](../../../../operations/backup-and-restore.md)) |
| Lưu trữ | Ảnh container và `compose.yaml`; phần mềm không tự thực hiện chuyển dữ liệu nào, nên vị trí dữ liệu là lựa chọn lưu trữ của bạn |

## Các khoảng trống bạn phải lấp

- Kiểm thử xâm nhập, rà soát mã độc lập và chứng nhận: chưa có cái nào.
- Việc lưu giữ mới chỉ được thực thi một phần; thao tác xóa phải được lên lịch (xem [lịch lưu giữ](../../../../governance/retention-schedule.md)).
- Công cụ dành cho chủ thể dữ liệu tìm thấy các tham chiếu, không tìm thấy tên được gõ vào văn bản tự do hay tệp tải lên.
- Nhật ký kiểm toán không chống được sự can thiệp của quản trị viên cơ sở dữ liệu.

## Đọc tiếp

[Sơ đồ luồng dữ liệu](../../../../governance/data-flow.md) · [Danh sách kiểm tra bảo đảm](../../../../governance/assurance-checklist.md) · [Triển khai](../../../../operations/deployment.md) · [INSTALL](../../../../INSTALL.md)
