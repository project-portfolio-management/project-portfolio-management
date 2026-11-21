# Con đường đến vận hành

*"Vận hành" có nghĩa là gì đối với phần mềm này, và phần mềm còn cách đó bao xa, từng hạng mục một.*

Ở đây, "vận hành" có nghĩa là một tổ chức chạy phần mềm cho công việc thực tế. Các kế hoạch thật, tên người thật và
các quyết định thật phụ thuộc vào nó, một sự cố ngừng hoạt động hay rò rỉ sẽ gây hậu quả, và có người phải chịu trách
nhiệm. Tại một cơ quan y tế lớn hoặc một bộ của chính phủ, vận hành còn có nghĩa là tổ chức đã vượt qua quy trình bảo
đảm của chính mình: kiểm thử bảo mật, đánh giá tác động bảo vệ dữ liệu, kiểm toán khả năng tiếp cận và, tại Dịch vụ
Y tế Quốc gia Anh (NHS), Tiêu chí Đánh giá Công nghệ Số (DTAC).

## Hiện trạng

- Không có chứng nhận, phê duyệt hay kiểm toán nào được tuyên bố. Trang này là ước tính riêng của dự án.
- "Y tế" ở đây có nghĩa là một công cụ quản lý danh mục và dự án **phi lâm sàng**: không có hồ sơ bệnh nhân và không có hỗ trợ ra quyết định lâm sàng.
- Dự án đề xuất bắt đầu bằng một thử nghiệm trên dữ liệu phi lâm sàng, không bí mật, do một tổ chức tự hoàn thành quy
  trình bảo đảm của mình thực hiện.

## Mức độ sẵn sàng, từng hạng mục

Mỗi con số là ước tính của dự án vào ngày 13 tháng 11 năm 2026, không phải một phép đo. Mỗi thanh trượt bắt đầu từ ước tính đó.
Di chuyển thanh trượt sẽ ghi lại quan điểm riêng của bạn trong lúc đọc; không có gì được lưu hay gửi đi.

- **Bảo đảm an ninh độc lập** — 0%. Chưa có kiểm thử xâm nhập, chưa có kiểm toán của bên thứ ba và chưa có chứng nhận. Bên mua hoặc nhà cung cấp phải đặt hàng thực hiện (RG-13).
- **Khả năng tiếp cận** — 40%. Một cuộc kiểm toán WCAG 2.2 AA tự động chạy trên mọi tuyến đường ở mỗi thay đổi. Chưa có kiểm thử thủ công, và chưa có kiểm thử với công nghệ hỗ trợ hay với người khuyết tật (RG-13).
- **Yêu cầu của chủ thể dữ liệu** — 50%. Quản trị viên có thể xuất và ẩn danh hóa bằng bí danh một người trên mọi tham chiếu trong lược đồ, sau khi xem trước (RG-6). Tên được gõ vào tiêu đề, bình luận hoặc tài liệu, tệp tải lên, ảnh chụp kiểm toán và bản sao lưu không được tìm thấy tự động.
- **Lưu giữ** — 35%. Một thao tác xóa, mặc định chạy thử, bao gồm thông báo, nhật ký gửi, khóa lũy đẳng và ảnh chụp xu hướng và mức độ sử dụng, với các khoảng thời gian đặt theo từng lần triển khai (RG-4). Kế hoạch, nhiệm vụ, bản ghi thời gian, tài liệu và nhật ký kiểm toán được giữ cho đến khi người vận hành xóa chúng.
- **Dấu xử lý** — 60%. Một phân loại có thể cấu hình ẩn các kế hoạch trên mức được phép của người gọi, kể cả quản trị viên, và gắn nhãn cho mọi phản hồi (RG-7). Dấu này không được đóng vào các tệp được tạo ra, và mức được phép chỉ đáng tin cậy bằng thuộc tính của nhà cung cấp danh tính.
- **An toàn tệp đính kèm** — 50%. Nội dung phải khớp với loại đã khai báo, chương trình bị từ chối, và một lần quét `clamd` tùy chọn sẽ đóng khi lỗi (RG-8). Không kèm bộ quét nào, và điểm nối này chỉ được thử với một bản thay thế.
- **Rà soát truy cập và truy cập khẩn cấp** — 45%. Một báo cáo về các quyền đang có hỗ trợ việc chứng nhận lại (RG-9), và truy cập khẩn cấp có thời hạn và cần nêu lý do (RG-10). Báo cáo cho thấy những gì đã được ghi lại, không phải những gì nhà cung cấp danh tính thực thi; không có tuyến nào ghi lại quyết định của người rà soát; và một thông báo truy cập khẩn cấp có thể nêu tên một người không nhận được gì trong khi quyền vẫn còn hiệu lực.
- **Hiệu năng và năng lực** — 40%. Một bài kiểm thử tải và chạy dài trên một máy nằm trong [`BENCHMARKS.md`](../../../BENCHMARKS.md): các yêu cầu thông thường được trả lời trong vài chục mili giây, một kế hoạch khoảng 12.700 nhiệm vụ lập lịch chậm, và tiến trình tăng lên khoảng 1 GB. Chưa có kế hoạch năng lực, chưa có kiểm thử với nhiều người dùng và chưa có lần chạy trên nhiều máy.
- **Phiên làm việc trên nhiều máy chủ giao diện** — 70%. Phiên được giữ trong bộ nhớ trừ khi cấu hình Redis (RG-12); khi dùng Redis, việc đăng nhập phụ thuộc vào nó.
- **Tính toàn vẹn của nhật ký kiểm toán** — 50%. Các thao tác ghi được kiểm toán, và một trigger cơ sở dữ liệu từ chối thay đổi và xóa, với những ngoại lệ được nêu rõ (RG-3). Quản trị viên cơ sở dữ liệu có thể gỡ trigger, và không có chuỗi băm.
- **Sao lưu và khôi phục** — 80%. Việc khôi phục được kiểm thử trong tích hợp liên tục, ở dạng thường và dạng mã hóa, theo các mục tiêu khôi phục nêu rõ (RG-5). Mã hóa là tùy chọn, và bài diễn tập chạy trên CI của chính dự án, không phải trên hạ tầng lưu trữ của bên triển khai.
- **Hỗ trợ và tính liên tục** — 10%. Một người bảo trì và không có thỏa thuận mức dịch vụ ([`GOVERNANCE.md`](../../../GOVERNANCE.md)).
- **Bản dịch** — 20%. Văn bản bằng các ngôn ngữ khác tiếng Anh là do máy viết cho đến khi người nói ngôn ngữ đó rà soát, và các nghĩa vụ ngôn ngữ theo luật định cần văn bản đã được rà soát ([tuyên bố về ngôn ngữ](../../../governance/language-statement.md)).
- **Sử dụng lâm sàng** — 0%. Chưa có hồ sơ an toàn lâm sàng nào được viết, và việc sử dụng lâm sàng nằm ngoài phạm vi.

## Lần triển khai đầu tiên nên làm gì

1. Hoàn thành [danh sách kiểm tra bảo đảm](../../../governance/assurance-checklist.md), bắt đầu bằng kiểm thử xâm nhập và kiểm toán khả năng tiếp cận.
2. Điền mọi dòng "Buyer to complete" trong [danh sách kiểm tra DTAC](../../../governance/dtac-checklist.md) và các dấu đánh dấu trong [DPIA](../../../governance/dpia.md).
3. Thống nhất thời hạn lưu giữ với người quản lý hồ sơ và lên lịch cho thao tác xóa.
4. Bật sao lưu có mã hóa và chạy bài diễn tập khôi phục trên hạ tầng lưu trữ của chính lần triển khai.
5. Giữ dữ liệu ở mức phi lâm sàng và dưới mức đánh dấu mà lần triển khai đã được bảo đảm.
6. Lập kế hoạch cho các khoảng trống nêu trên, hoặc cấp vốn để loại bỏ chúng.

Bằng chứng cho từng hạng mục nằm trong [bản đánh giá](../../../governance/production-readiness.md).
