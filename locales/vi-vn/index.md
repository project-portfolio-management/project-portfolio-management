# Quản lý danh mục dự án

*Sổ đăng ký các công việc được cấp vốn, có phát hiện trùng lặp và không gian làm việc quản lý dự án.*

Quản lý danh mục dự án là phần mềm mã nguồn mở để ghi lại các kế hoạch mà một tổ chức cấp vốn và để quản lý công việc trong các kế hoạch đó. Phần mềm bao quát nhiệm vụ, lịch trình, chi phí, nguồn lực, rủi ro và quản trị. Phần mềm so sánh các kế hoạch để nhận ra những kế hoạch có thể trùng lặp và hiển thị bằng chứng cho từng kết quả khớp. Khi không thể tính được một con số, phần mềm nêu rõ lý do.

## Lợi ích chính

- **Một hồ sơ danh mục duy nhất.** Danh mục, chương trình, dự án và sản phẩm dùng chung một loại bản ghi, và bất kỳ kế hoạch nào cũng có thể chứa kế hoạch khác. Báo cáo lấy dữ liệu từ một danh sách chung cho mọi phòng ban.
- **Phát hiện trùng lặp.** Việc đối chiếu so sánh các kế hoạch được ghi dưới những tên khác nhau và hiển thị bằng chứng cho từng kết quả, để người rà soát có thể hành động trước khi cả hai kế hoạch được cấp vốn.
- **Số liệu có thể truy vết.** Một con số bị thiếu được hiển thị kèm lý do thiếu. Một kế hoạch chưa được đánh giá được hiển thị là "chưa đánh giá".
- **Nhật ký kiểm toán.** Mọi thay đổi đều được ghi vào nhật ký kiểm toán. Người đưa ra yêu cầu thay đổi không thể tự phê duyệt yêu cầu đó.
- **Kiểm soát truy cập.** Đăng nhập được bật theo mặc định, và dịch vụ không chạy ở môi trường vận hành khi đăng nhập bị tắt.
- **Triển khai và cấp phép.** Phần mềm do bạn tự lưu trữ và được cung cấp theo năm giấy phép mã nguồn mở. Có thể nhập công việc từ Jira, Asana và Microsoft Project.

[Xem tất cả lợi ích](topics/lợi-ích/index.md)

## Các mảng chức năng

- **Kế hoạch và đối chiếu.** Sổ đăng ký đệ quy, phát hiện trùng lặp có giải thích được và gộp bản ghi.
- **Triển khai.** Bảng kanban, biểu đồ Gantt, lịch, cột mốc, đường găng và chuỗi găng.
- **Tài chính.** Ngân sách, sổ cái cấp vốn, giá trị thu được, và rủi ro về chi phí và lịch trình.
- **Nguồn lực.** Nhóm kỹ năng, bản đồ nhiệt năng lực và nhân sự chủ chốt.
- **Chiến lược.** Tiếp nhận công việc, ý tưởng, mục tiêu, mục tiêu và kết quả then chốt, và các kịch bản.
- **Quản trị.** Rủi ro, vấn đề, các buổi rà soát cổng, leo thang và gói báo cáo hằng tháng cho ban giám đốc.

[Xem tất cả tính năng](topics/tính-năng/index.md)

## Người dùng dự kiến

- **Văn phòng danh mục.** Báo cáo xuyên suốt các chương trình và dự án.
- **Quản lý dự án và quản lý triển khai.** Lập lịch, quản lý chi phí và rủi ro.
- **Thành viên nhóm.** Chế độ xem cá nhân về công việc được giao.
- **Lãnh đạo cấp cao.** Các [bản tóm tắt theo vai trò](for/) nêu những gì phần mềm cung cấp cho từng vai trò, các giới hạn của nó và các quyết định cần đưa ra.

## Tình trạng hiện tại

Phần mềm đang ở giai đoạn đầu. Phần mềm chỉ có một người bảo trì, chưa được bên thứ ba kiểm toán và chưa có thỏa thuận hỗ trợ. Trang [con đường đến vận hành](path-to-production/index.md) nêu từng hạng mục cần có để đưa phần mềm vào sử dụng thực tế, kèm ước tính cho mỗi hạng mục. Các hạn chế đã biết được liệt kê ở [trang so sánh](topics/so-sánh/index.md).

## Bản trình diễn

Bản trình diễn cục bộ dùng một cơ quan giao thông hư cấu và cần có Git và Podman. Các bước nằm trong [hướng dẫn](topics/hướng-dẫn/index.md).
