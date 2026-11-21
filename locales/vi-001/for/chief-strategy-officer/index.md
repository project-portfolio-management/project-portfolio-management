# Dành cho Giám đốc chiến lược

*Giám đốc chiến lược (CSO): phần mềm gắn chiến lược với công việc ra sao, nó cho bạn thấy điều gì, và những gì nó chưa làm được.*

## Phần mềm làm gì cho chiến lược

- **Một sổ đăng ký duy nhất cho mọi thứ được cấp vốn.** Danh mục, chương trình, dự án và sản phẩm là một bản ghi đệ quy, nên toàn bộ tài sản có thể được đọc như một cây ([mục đích và tầm nhìn](../../../../spec/01-purpose-and-vision.md)).
- **Phát hiện trùng lặp.** Việc đối chiếu có giải thích được cho thấy khi hai bộ phận của tổ chức đã cùng phê duyệt một sáng kiến, trước khi cả hai được cấp vốn.
- **Mục tiêu chiến lược và bản đồ danh mục.** Bạn đánh dấu các mục tiêu là mục tiêu chiến lược và nêu một mục tiêu chính cho từng chương trình. `GET /api/portfolio-map` và gói báo cáo hằng tháng cho ban giám đốc nhóm các chương trình theo mục tiêu chính. Một chương trình xuất hiện một lần, với các mục tiêu khác của nó được nêu tên; một chương trình không có mục tiêu chính được liệt kê dưới "không có mục tiêu chiến lược", và các tổng số không bao giờ bị tính hai lần.
- **Mức độ tin cậy về tiến độ bên cạnh trạng thái tính toán.** Con người nêu mức tin cậy của họ về việc triển khai; phần mềm hiển thị nó cạnh trạng thái đỏ/hổ phách/xanh được tính toán và gắn cờ khi chênh lệch hai bậc. Một kế hoạch chưa ai đánh giá hiển thị "chưa đánh giá", không bao giờ là xanh.
- **Một gói báo cáo hằng tháng đã đóng băng.** Những gì ban giám đốc được thông báo được giữ nguyên như khi phát hành, với xu hướng được đo so với gói đã phát hành trước đó.

## Những gì phần mềm chưa làm

- Gói báo cáo chưa có các quyết định tại cổng, cấp vốn theo năm tài chính, biểu đồ sốt vùng đệm, giá trị thu được, lợi ích đã được xác nhận hay bản đồ nhiệt năng lực (`DP-5` đến `DP-10` trong [`tasks.md`](../../../../tasks.md)). Gói báo cáo nêu rõ những phần nào chưa được xây dựng.
- "Chương trình" có nghĩa là một kế hoạch có loại là `Program`, vì dữ liệu không có định nghĩa nào khác; mục tiêu "cấp cao nhất" là những mục tiêu bạn đánh dấu, vì các mục tiêu không có cấp bậc.
- Chưa có lập kế hoạch theo kịch bản, và chưa có màn hình cho bản đồ hay gói báo cáo: chúng có sẵn dưới dạng dữ liệu.
- Phần mềm chưa được chứng nhận và chỉ có một người bảo trì ([con đường đến vận hành](../../path-to-production/index.md)).

## Những quyết định thuộc về bạn

Những mục tiêu nào là mục tiêu chiến lược; mục tiêu chính của từng chương trình; ai nêu và ai đánh giá độc lập mức độ tin cậy về tiến độ; ban giám đốc nên thấy xu hướng của bao nhiêu tháng.

## Đọc tiếp

[Lộ trình](../../../../spec/15-roadmap.md) · [Các câu hỏi còn mở](../../../../spec/16-open-questions.md) · [Phạm vi](../../../../spec/02-scope.md)
