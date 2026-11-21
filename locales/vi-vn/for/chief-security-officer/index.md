# Dành cho Giám đốc an ninh

*Giám đốc an ninh (CSO): những gì phần mềm thực thi, những gì dự án đã kiểm chứng, và những gì bạn phải tự kiểm thử và vận hành.*

## Những gì phần mềm thực thi

| Mảng | Hành vi |
|---|---|
| Xác thực | Đăng nhập được bật theo mặc định. Dịch vụ sẽ không khởi động nếu thiếu khóa xác minh token, và sẽ không khởi động ở môi trường vận hành khi đăng nhập bị tắt. Đăng nhập qua một nhà cung cấp danh tính như Microsoft Entra ID; xác thực đa yếu tố do nhà cung cấp đảm nhiệm ([hướng dẫn đăng nhập](../../../../operations/sign-in-with-entra.md)) |
| Ủy quyền | Chính sách theo thuộc tính dựa trên các thuộc tính của token. Các kế hoạch riêng tư bị loại khỏi mọi số liệu tổng hợp đối với tất cả mọi người; các dấu xử lý ẩn các kế hoạch trên mức được phép của người gọi, kể cả quản trị viên |
| Truy cập khẩn cấp | Truy cập vào các kế hoạch riêng tư cần đủ điều kiện, lý do nêu rõ, tối đa bốn giờ, và một người thứ hai được báo; mọi yêu cầu đều được kiểm toán |
| Phiên làm việc | Thời gian chờ khi không hoạt động (mặc định 30 phút) và giới hạn phiên cứng |
| Truyền tải và trình duyệt | Các tiêu đề bảo mật, Content Security Policy nghiêm ngặt, danh sách cho phép CORS, giới hạn tần suất theo từng người gọi |
| Gửi đi | Các địa chỉ đích do người dùng cung cấp bị giới hạn ở các địa chỉ `https` công khai |
| Tải lên | Nội dung phải khớp với loại đã khai báo; chương trình bị từ chối; một lần quét `clamd` tùy chọn sẽ đóng khi lỗi |
| Kiểm toán | Nhật ký kiểm toán từ chối thay đổi và xóa trong cơ sở dữ liệu |
| Bản ghi đóng băng | Các gói báo cáo ban giám đốc đã phát hành và các đánh giá mức độ tin cậy về tiến độ là bất biến hoặc chỉ thêm vào được nhờ trigger cơ sở dữ liệu |
| Chuỗi cung ứng | `cargo deny`, bảng kê vật liệu phần mềm (SBOM), nguồn gốc bản dựng khi phát hành, một [chính sách bảo mật](../../../../SECURITY.md) |

## Những gì chưa làm

- Chưa có kiểm thử xâm nhập, kiểm toán của bên thứ ba, rà soát mã độc lập hay chứng nhận.
- Không kèm bộ quét mã độc nào; điểm nối `clamd` chỉ được thử với một bản thay thế.
- Quản trị viên cơ sở dữ liệu có thể vô hiệu hóa nhật ký kiểm toán; không có chuỗi băm.
- Dấu xử lý chỉ ẩn và gắn nhãn; chúng không mã hóa, và dấu không được đóng vào các tệp được tạo ra.
- Phiên được giữ trong bộ nhớ trừ khi cấu hình Redis.
- Công cụ dành cho chủ thể dữ liệu tìm thấy các tham chiếu, không tìm thấy tên được gõ vào văn bản tự do, tệp tải lên hay bản sao lưu.

## Vận hành

Sao lưu có khôi phục đã được kiểm thử, mã hóa tùy chọn và các mục tiêu khôi phục nêu rõ ([gói vận hành](../../../../operations/README.md)). [Sổ tay vận hành](../../../../operations/runbook.md) nêu các bước kỹ thuật xử lý sự cố; quy trình xử lý sự cố và vi phạm dữ liệu là của bạn. Hãy báo cáo lỗ hổng một cách riêng tư như [SECURITY.md](../../../../SECURITY.md) mô tả; mục tiêu là xác nhận trong 5 ngày làm việc, không phải bảo đảm.

## Những quyết định thuộc về bạn

Ai thực hiện kiểm thử xâm nhập và khi nào; việc truy cập có điều kiện và ánh xạ vai trò của nhà cung cấp danh tính; mức đánh dấu mà lần triển khai này được bảo đảm; bộ quét đứng sau điểm nối tệp đính kèm; bao lâu chứng nhận lại quyền truy cập một lần.

## Đọc tiếp

[Danh sách kiểm tra bảo đảm](../../../../governance/assurance-checklist.md) · [Con đường đến vận hành](../../path-to-production/index.md) · [Luồng dữ liệu](../../../../governance/data-flow.md)
