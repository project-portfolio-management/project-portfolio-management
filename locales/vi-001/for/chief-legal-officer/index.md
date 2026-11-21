# Dành cho Giám đốc pháp chế

*Giám đốc pháp chế: giấy phép, bảo vệ dữ liệu, khả năng tiếp cận, nghĩa vụ về ngôn ngữ và những gì dự án không bảo đảm. Đây không phải là tư vấn pháp lý.*

## Giấy phép

Được cung cấp theo bất kỳ một trong các giấy phép MIT, Apache-2.0, BSD-3-Clause, GPL-2.0-only hoặc GPL-3.0-only, hoặc theo điều khoản riêng khi có thỏa thuận. Lựa chọn và văn bản giấy phép nằm trong [`LICENSE.md`](../../../../LICENSE.md) và `LICENSE/`; một bài kiểm thử giữ cho mọi tệp kê khai khớp với chúng.

## Bảo vệ dữ liệu (luật áp dụng cho bạn)

Được viết cho mọi luật bảo vệ dữ liệu: [đặc tả tuân thủ §12.2a](../../../../spec/12-compliance.md) ánh xạ các luật phổ biến theo khu vực (GDPR và GDPR của Anh, LGPD, PIPL, đạo luật DPDP, PIPEDA, CCPA và các luật khác) với những gì mỗi luật đòi hỏi ở đây, dựa trên hiểu biết chung và không được đối chiếu với văn bản của từng luật. Các tài liệu được điền sẵn để bạn điều chỉnh; mỗi dấu `‹buyer›` là một quyết định thuộc về bạn (bên kiểm soát, cơ sở pháp lý, thời hạn lưu giữ, bên xử lý, ký duyệt):

- [Mẫu DPIA](../../../../governance/dpia.md) và [hồ sơ xử lý](../../../../governance/record-of-processing.md)
- [Bản kê dữ liệu cá nhân](../../../../governance/personal-data-inventory.csv): mọi cột có thể chứa dữ liệu cá nhân
- [Lịch lưu giữ](../../../../governance/retention-schedule.md): các đề xuất, kèm những gì mã nguồn thực thi hiện nay
- [Sơ đồ luồng dữ liệu](../../../../governance/data-flow.md)

Phần mềm lưu con người dưới dạng các tham chiếu ẩn danh cùng thời gian đã ghi, khả năng sẵn sàng và đơn giá chi phí, điều này gần với việc giám sát nên cần một DPIA. Phần mềm cung cấp việc xuất dữ liệu của chủ thể dữ liệu và việc xóa bằng ẩn danh hóa bằng bí danh hoặc, nếu bên triển khai chọn, bằng xóa hẳn (chỉ quản trị viên, xem trước trước; WW-1.8). Các gói báo cáo ban giám đốc đã đóng băng và lịch sử mức độ tin cậy chỉ thêm vào được sẽ bị xóa trắng tên người soạn hoặc đánh giá, không bao giờ bị xóa, nên một yêu cầu xóa có thể được đáp ứng mà không phá vỡ hồ sơ. Phần mềm **không** tìm thấy tên được gõ vào văn bản tự do, và các bản sao lưu chứa những bản ghi mà ứng dụng đã xóa.

## Các khung quy định

HIPAA và các chế độ chứng nhận dữ liệu y tế của Hoa Kỳ **không liên quan** (không có dữ liệu bệnh nhân); chế độ về thiết bị lâm sàng không liên quan. [Danh sách kiểm tra DTAC](../../../../governance/dtac-checklist.md) trả lời Tiêu chí Đánh giá Công nghệ Số của NHS Anh, một chương trình quốc gia được giữ làm ví dụ minh họa; với các chương trình của quốc gia và ngành khác (ISO/IEC 27001, SOC 2, các chương trình đám mây quốc gia và khu vực công), hãy bắt đầu từ [danh sách kiểm tra bảo đảm](../../../../governance/assurance-checklist.md). Không có gì được chứng nhận.

## Khả năng tiếp cận và ngôn ngữ

WCAG 2.2 AA được kiểm thử tự động trên mọi tuyến đường; chưa có kiểm thử thủ công hay với công nghệ hỗ trợ ([tuyên bố](../../../../governance/accessibility-statement.md)). Tiếng Wales và các bản dịch khác là bản nháp đang chờ biên dịch viên chuyên nghiệp ([tuyên bố về ngôn ngữ](../../../../governance/language-statement.md)); điều này liên quan nếu bạn có nghĩa vụ ngôn ngữ theo luật định.

## Thẩm định nhà cung cấp

Một người bảo trì, không có hỗ trợ theo hợp đồng, không tuyên bố bảo hành, các công cụ AI được dùng trong phát triển và được công khai trong [tuyên bố về AI](../../../../AI_STATEMENT.md). Sản phẩm không chứa mô hình AI nào. Báo cáo lỗ hổng: [SECURITY.md](../../../../SECURITY.md).

## Những quyết định thuộc về bạn

Cơ sở pháp lý và thời hạn lưu giữ; có cần quy trình lưu giữ vì mục đích pháp lý hay không (chưa có quy trình nào được xây dựng); điều khoản hợp đồng và hỗ trợ với người bảo trì.

## Đọc tiếp

[Danh sách kiểm tra bảo đảm](../../../../governance/assurance-checklist.md) · [Mục lục gói quản trị](../../../../governance/README.md)
