# So sánh

*Tóm tắt về mối quan hệ giữa phần mềm này và các sản phẩm khác.*

Các sản phẩm dưới đây phục vụ những mục đích khác nhau và không được chấm điểm. Các ghi chú này dựa trên tài liệu công khai của từng dự án. **Không có sản phẩm nào khác được cài đặt hay chạy thử**, và mỗi sản phẩm có thể đã thay đổi kể từ đó. Cần kiểm tra tài liệu sản phẩm hiện hành trước khi đưa ra quyết định.

| Dự án | Đó là gì | Dự án này khác ở điểm nào |
| --- | --- | --- |
| [Leantime](https://github.com/leantime/leantime) | Quản lý dự án cho những người không phải là quản lý dự án | Dự án này còn có các chức năng lập lịch, chi phí và quản trị mà một văn phòng danh mục sử dụng |
| [Redmine](https://www.redmine.org/) | Một công cụ theo dõi đa dụng, có thể cấu hình, với các loại vấn đề, trạng thái và vai trò do tổ chức tự định nghĩa | Các loại vấn đề, trạng thái và quy trình do tổ chức tự định nghĩa. Vai trò mang theo quyền đối với vấn đề, được gán theo từng kế hoạch, trong phạm vi mà đăng nhập của một người cho phép. Xem [so sánh với Redmine](../../../en-001/compare/redmine/index.md) |
| Huly Platform (qua bản phái sinh Platform Collective) | Một khung ứng dụng doanh nghiệp chạy trên nhiều dịch vụ | Huly là nền tảng nhiều dịch vụ. Dự án này là một dịch vụ duy nhất chạy trên PostgreSQL |
| [ProjectLibre](https://sourceforge.net/p/projectlibre/) | Một giải pháp thay thế Microsoft Project trên máy tính, cho một người quản lý và một dự án | Dự án này là dịch vụ được lưu trữ, nhiều người dùng, có kiểm soát truy cập |

## So sánh theo sản phẩm

Các trang so sánh riêng (bằng tiếng Anh): [Trello](../../../en-001/compare/trello/index.md), [Jira](../../../en-001/compare/jira/index.md), [Asana](../../../en-001/compare/asana/index.md), [Microsoft Project](../../../en-001/compare/microsoft-project/index.md), [Planview](../../../en-001/compare/planview/index.md), [Smartsheet](../../../en-001/compare/smartsheet/index.md), [Leantime](../../../en-001/compare/leantime/index.md), [Redmine](../../../en-001/compare/redmine/index.md), [ProjectLibre](../../../en-001/compare/projectlibre/index.md).

## Khả năng tương tác

- **Jira và Asana:** nhập CSV, kèm bản xem trước và lý do nêu rõ cho mọi mục bị loại.
- **Microsoft Project:** nhập và xuất XML MSPDI, bao gồm nhiệm vụ, cấu trúc phân cấp, ngày tháng, khoảng thời gian, ràng buộc, quan hệ phụ thuộc và lịch. Người được giao việc, nguồn lực và đường cơ sở bị loại, và tệp có ghi lại điều này.

## Khác biệt về thiết kế

- **Lý do được nêu rõ.** Một con số bị thiếu được hiển thị kèm lý do, ví dụ "không có dự báo, vì…".
- **Phát hiện trùng lặp có giải thích được.** Việc đối chiếu hoạt động với mọi loại kế hoạch và hiển thị bằng chứng của nó.
- **Mặc định an toàn.** Đăng nhập được bật theo mặc định, và tài liệu bảo mật nằm trong kho mã nguồn.
- **Khoảng thời gian tường minh.** Mỗi khoảng thời gian được nêu rõ theo ngày dương lịch hay ngày làm việc.
- **Giới hạn được ghi lại.** Danh sách việc cần làm ghi lại các giới hạn của từng hạng mục.

## Các hạn chế đã biết

- Không có ứng dụng di động hay ứng dụng máy tính.
- Không có đồng chỉnh sửa theo thời gian thực.
- Chỉ có một người bảo trì, lịch sử vận hành còn hạn chế và chưa được bên thứ ba kiểm toán.
- Bài kiểm thử tải đã công bố được chạy trên một máy. Nó cho biết bậc độ lớn và không phải là kế hoạch năng lực.
- Các bản dịch là bản nháp đang chờ rà soát.
- Không có màn hình để soạn thảo hoặc quyết định các yêu cầu thay đổi.
- Không có hệ sinh thái tiện ích mở rộng hay chợ ứng dụng.

Chi tiết thêm có trong [ghi chú so sánh](https://github.com/project-portfolio-management/project-portfolio-management/blob/main/COMPARISONS.md). Có thể gửi đính chính dưới dạng một vấn đề (issue) kèm nguồn.
