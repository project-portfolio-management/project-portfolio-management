# Lợi ích

*Các lợi ích chính đối với một tổ chức, theo từng mảng.*

## Tầm nhìn về danh mục

Thông tin danh mục thường nằm rải rác ở nhiều nơi. Trong phần mềm này, mọi danh mục, chương trình, dự án và sản phẩm đều là một loại bản ghi, gọi là **kế hoạch**. Bất kỳ kế hoạch nào cũng có thể chứa kế hoạch khác, ở độ sâu tùy ý, nên cấu trúc có thể phản ánh đúng tổ chức. Một nhãn như "Dự án" chỉ mang tính mô tả và không đặt ra hạn chế nào cho kế hoạch.

## Giảm trùng lặp

Hai phòng ban có thể cùng phê duyệt một sáng kiến dưới những tên khác nhau. Bộ đối chiếu so sánh các thuộc tính xác định một kế hoạch: tên, mục tiêu, chủ sở hữu, nhà tài trợ, kế hoạch cấp trên, khung thời gian, từ khóa và mã định danh. Bộ đối chiếu hoạt động với mọi loại kế hoạch và hiển thị bằng chứng cho từng kết quả khớp, còn quyết định do con người đưa ra. Hai bản ghi của cùng một kế hoạch có thể được gộp lại.

## Chất lượng báo cáo

- Một con số bị thiếu được hiển thị kèm lý do, ví dụ "không có dự báo, vì…".
- Một kế hoạch chưa được đánh giá được hiển thị là "chưa đánh giá".
- Mức độ tin cậy về tiến độ do con người ghi nhận được hiển thị bên cạnh trạng thái tính toán, và hai thứ này được tách biệt.
- Các kế hoạch riêng tư bị loại khỏi mọi tổng số và số đếm đối với mọi người dùng. Các bài kiểm thử kiểm tra quy tắc này.
- Mỗi khoảng thời gian đều nêu rõ là tính theo ngày dương lịch hay ngày làm việc.

## Hồ sơ quyết định

Mọi thay đổi đều được kiểm toán, và không thể sửa nhật ký kiểm toán thông qua dịch vụ. Người đưa ra yêu cầu thay đổi không thể phê duyệt yêu cầu đó. Gói báo cáo hằng tháng cho ban giám đốc được soạn một lần, do một người có tên cụ thể phát hành, rồi được cơ sở dữ liệu đóng băng. Việc đính chính được phát hành dưới dạng một phiên bản mới kèm ghi chú.

## Bảo mật

Đăng nhập được bật theo mặc định, và dịch vụ không khởi động ở môi trường vận hành khi đăng nhập bị tắt. Kho mã nguồn bao gồm các tiêu đề bảo mật, giới hạn tần suất, kế hoạch khôi phục với các mục tiêu nêu rõ và một gói quản trị thông tin.

## Kiểm soát và khả năng chuyển đổi

- **Cấp phép.** Phần mềm là mã nguồn mở, theo lựa chọn MIT, Apache-2.0, BSD-3-Clause, GPL-2.0, GPL-3.0, OGL-UK-3.0 hoặc CC-BY-NC-SA-4.0.
- **Lưu trữ.** Phần mềm do bạn tự lưu trữ: một dịch vụ Rust chạy trên PostgreSQL, với giao diện SvelteKit.
- **Nhập dữ liệu.** Có thể nhập tệp CSV từ Jira và Asana, kèm bản xem trước và lý do nêu rõ cho mọi mục bị loại. Có thể nhập và xuất XML của Microsoft Project.
- **Ngôn ngữ.** Giao diện có sẵn ở mười ngôn ngữ và khu vực, trong đó có tiếng Ả Rập viết từ phải sang trái. Các bản dịch là bản nháp đang chờ người nói ngôn ngữ đó rà soát.

## Các trang liên quan

Xem thêm [tính năng](../tính-năng/index.md), [so sánh](../so-sánh/index.md) và [hướng dẫn](../hướng-dẫn/index.md).
