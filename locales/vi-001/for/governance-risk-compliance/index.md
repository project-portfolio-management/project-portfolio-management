# Quản trị, rủi ro, tuân thủ

*Dành cho những người điều hành công tác quản trị, rủi ro và tuân thủ: phần mềm giúp bạn chứng minh điều gì, thực thi điều gì, và những gì bạn phải tự cung cấp. Không phải tư vấn pháp lý.*

## Quản trị

- **Các quyết định được ghi lại.** Yêu cầu thay đổi cần một người phê duyệt có tên, không phải người yêu cầu, một lý do nêu rõ dài ít nhất 10 ký tự, và được kiểm toán theo từng người thực hiện; cơ sở dữ liệu từ chối một lần phê duyệt thiếu người quyết định và lý do.
- **Một gói báo cáo hằng tháng cho ban giám đốc đã đóng băng.** Soạn một lần, do một người phát hành, bất biến sau khi phát hành; việc đính chính là một lần phát hành lại kèm ghi chú. Quyền đọc được cấp theo đối tượng.
- **Mức độ tin cậy về tiến độ và ngoại lệ.** Mức tin cậy do con người nêu chỉ thêm vào được và hiển thị bên cạnh trạng thái tính toán, không bao giờ trộn lẫn. Một mức đánh giá đỏ hoặc hổ phách-đỏ mở một ngoại lệ với đồng hồ 48 giờ (có thể cấu hình) mà chỉ một quyết định được ghi lại mới đóng được.
- **Quản trị dự án của chính phần mềm:** [GOVERNANCE.md](../../../../GOVERNANCE.md), [kiểm soát thay đổi](../../../../spec/18-change-control.md), [tuyên bố về AI](../../../../AI_STATEMENT.md).

## Rủi ro

- Rủi ro, vấn đề và yêu cầu thay đổi theo từng kế hoạch; một sổ đăng ký vấn đề trên các kế hoạch có thể đọc được.
- Dữ liệu thiếu được hiển thị là "chưa đánh giá", không bao giờ là xanh.
- Các rủi ro còn mở của chính dự án (chưa có kiểm thử xâm nhập, lưu giữ một phần, một người bảo trì, bản dịch là bản nháp) nằm trên trang [con đường đến vận hành](../../path-to-production/index.md), từng hạng mục kèm ước tính cho mỗi hạng mục.

## Tuân thủ

| Mảng | Ở đâu | Trạng thái |
|---|---|---|
| Bảo vệ dữ liệu (luật áp dụng; [§12.2a](../../../../spec/12-compliance.md) ánh xạ GDPR, GDPR của Anh, LGPD, PIPL, DPDP, PIPEDA, CCPA và các luật khác) | [DPIA](../../../../governance/dpia.md), [hồ sơ xử lý](../../../../governance/record-of-processing.md), [bản kê dữ liệu cá nhân](../../../../governance/personal-data-inventory.csv), [lịch lưu giữ](../../../../governance/retention-schedule.md), [luồng dữ liệu](../../../../governance/data-flow.md) | Đã điền sẵn; các dấu `‹buyer›` là của bạn; việc lưu giữ chỉ được thực thi một phần |
| DTAC của NHS Anh (một chương trình quốc gia, một ví dụ minh họa; các chương trình khác bắt đầu từ [danh sách kiểm tra bảo đảm](../../../../governance/assurance-checklist.md)) | [Danh sách kiểm tra DTAC](../../../../governance/dtac-checklist.md) | Đã trả lời từng phần; các dòng của bên mua và nhà cung cấp còn mở |
| Khả năng tiếp cận | [tuyên bố](../../../../governance/accessibility-statement.md) | WCAG 2.2 AA tự động; chưa có kiểm thử thủ công |
| Nghĩa vụ về ngôn ngữ | [tuyên bố về ngôn ngữ](../../../../governance/language-statement.md) | Các bản dịch là bản nháp |
| ISO/IEC 27001 | [đặc tả tuân thủ](../../../../spec/12-compliance.md) | Các biện pháp kiểm soát phía triển khai; chưa được chứng nhận |
| HIPAA, EHDS, ONC/HTI, IEC 62304 | [đặc tả tuân thủ](../../../../spec/12-compliance.md) §12.4 | Không liên quan (không có dữ liệu bệnh nhân); được nêu ra, không được dựng lên |

## Bằng chứng và bảo đảm

[Danh sách kiểm tra bảo đảm](../../../../governance/assurance-checklist.md) liệt kê 14 mục mà một người đánh giá sẽ hỏi, kèm người phụ trách và trạng thái. Phần lớn còn mở và thuộc về bên triển khai: kiểm thử xâm nhập, kiểm toán khả năng tiếp cận, diễn tập khôi phục sau thảm họa, bảo đảm hạ tầng lưu trữ, cấu hình nhà cung cấp danh tính, chứng nhận lại quyền truy cập, quy trình xử lý sự cố, quản lý hồ sơ. Nhật ký kiểm toán từ chối thay đổi và xóa trong cơ sở dữ liệu, nhưng quản trị viên cơ sở dữ liệu có thể vô hiệu hóa nó; không có chuỗi băm.

## Những quyết định thuộc về bạn

Cơ sở pháp lý và thời hạn lưu giữ; khoảng thời gian ngoại lệ; mức đánh dấu cho lần triển khai; ai rà soát quyền truy cập và bao lâu một lần; có cần quy trình lưu giữ vì mục đích pháp lý hay không (chưa có quy trình nào được xây dựng).

## Đọc tiếp

[Mục lục gói quản trị](../../../../governance/README.md) · [SECURITY](../../../../SECURITY.md) · [Đặc tả tuân thủ](../../../../spec/12-compliance.md)
