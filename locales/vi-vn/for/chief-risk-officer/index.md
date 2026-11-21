# Dành cho Giám đốc rủi ro

*Giám đốc rủi ro (CRO): những rủi ro mà phần mềm giúp bạn nhìn thấy, và những rủi ro khi sử dụng nó.*

## Rủi ro mà phần mềm giúp bạn quản lý

- **Rủi ro, vấn đề và yêu cầu thay đổi** được ghi theo từng kế hoạch; một sổ đăng ký vấn đề trên các kế hoạch có thể đọc được; yêu cầu thay đổi với phân tách nhiệm vụ.
- **Mức độ tin cậy về tiến độ và ngoại lệ:** mức tin cậy do con người nêu được tách khỏi RAG tính toán, được gắn cờ khi hai bên lệch nhau hai bậc, với lịch sử chỉ thêm vào được, và một đồng hồ ngoại lệ 48 giờ không khởi động lại khi mức đánh giá xấu đi và chỉ được đóng bằng một quyết định được ghi lại.
- **Trung thực về chỗ thiếu:** dữ liệu thiếu hiển thị là "chưa đánh giá" hoặc null kèm lý do, không bao giờ là xanh hay bằng không.
- **Một gói báo cáo hằng tháng đã đóng băng, có phiên bản,** để những gì đã báo cáo với ban giám đốc có thể được trình bày lại đúng như khi phát hành.

## Rủi ro khi sử dụng (sổ đăng ký riêng của dự án)

| Rủi ro | Trạng thái |
|---|---|
| Chưa có kiểm thử xâm nhập, kiểm toán của bên thứ ba hay chứng nhận | Còn mở: bên mua phải đặt hàng ([danh sách kiểm tra bảo đảm](../../../../governance/assurance-checklist.md)) |
| Việc lưu giữ chỉ được thực thi với một phần dữ liệu | Còn mở; thao tác xóa phải được lên lịch |
| Công cụ dành cho chủ thể dữ liệu không tìm trong văn bản tự do, tệp đính kèm hay bản sao lưu | Còn mở |
| Quản trị viên cơ sở dữ liệu có thể vô hiệu hóa nhật ký kiểm toán; không có chuỗi băm | Được nêu trong [DPIA](../../../../governance/dpia.md) |
| Dấu xử lý chỉ ẩn và gắn nhãn, không mã hóa | Được nêu |
| Quét tệp đính kèm cần một bộ quét do bên triển khai vận hành | Còn mở |
| Một người bảo trì, không có SLA | Còn mở |
| Các bản dịch là bản nháp | Còn mở |
| Chỉ dành cho phi lâm sàng; không có hồ sơ an toàn lâm sàng | Không nhập nội dung lâm sàng |

## Các biện pháp kiểm soát mà phần mềm thực thi

Đăng nhập theo mặc định, giới hạn tần suất, các tiêu đề bảo mật nghiêm ngặt, thời gian chờ phiên khi không hoạt động (30 phút) và giới hạn phiên cứng, các địa chỉ gửi đi bị giới hạn ở `https` công khai, tệp đính kèm được kiểm tra chữ ký, các dòng kiểm toán bất biến, truy cập khẩn cấp có kiểm toán. Mỗi biện pháp đều có một bài kiểm thử được dẫn trong [con đường đến vận hành](../../path-to-production/index.md).

## Những quyết định thuộc về bạn

Khẩu vị rủi ro cho một thử nghiệm; khoảng thời gian ngoại lệ (mặc định 48 giờ); mức đánh dấu mà lần triển khai được bảo đảm; ai rà soát danh sách truy cập và bao lâu một lần.

## Đọc tiếp

[Con đường đến vận hành](../../path-to-production/index.md) · [DPIA](../../../../governance/dpia.md) · [Tuân thủ](../../../../spec/12-compliance.md) · [SECURITY](../../../../SECURITY.md)
