# Dành cho Tổng giám đốc điều hành

*Tổng giám đốc điều hành (CEO): phần mềm này là gì, nó mang lại gì cho bạn, nó không mang lại gì, và những quyết định thuộc về bạn.*

## Phần mềm là gì, trong một đoạn

Một sổ đăng ký mọi **kế hoạch** mà tổ chức cấp vốn (danh mục, chương trình, dự án, sản phẩm và các loại khác, dùng chung một bản ghi đệ quy), với một không gian làm việc quản lý dự án bên dưới (nhiệm vụ, lịch trình, chi phí, nguồn lực, rủi ro, vấn đề, yêu cầu thay đổi, quản trị). Phần mềm tìm các sáng kiến trùng lặp bằng cách đối chiếu có giải thích được, nên khi hai phòng ban cùng phê duyệt một công việc, điều đó lộ ra trước khi cả hai được cấp vốn. Xem [mục đích và tầm nhìn](../../../../spec/01-purpose-and-vision.md).

## Những gì bạn có thể tin cậy

- **Một nơi duy nhất để hỏi "chúng ta đang làm gì?"** Các kế hoạch lồng nhau ở độ sâu tùy ý; các kế hoạch riêng tư nằm ngoài mọi tổng số và số đếm, đối với tất cả mọi người, theo một quy tắc mà các bài kiểm thử kiểm tra.
- **Một gói báo cáo hằng tháng cho ban giám đốc** được soạn một lần, do một người có tên cụ thể phát hành, rồi được **cơ sở dữ liệu đóng băng**: gói đã phát hành không thể sửa; việc đính chính là một phiên bản mới được phát hành lại kèm ghi chú. Hiện nay gói gồm bản đồ danh mục theo mục tiêu chiến lược, mức độ tin cậy về tiến độ, các ngoại lệ, vấn đề và yêu cầu thay đổi.
- **Mức độ tin cậy về tiến độ do con người nêu, hiển thị bên cạnh trạng thái tính toán, không bao giờ trộn lẫn với nó.** Một kế hoạch chưa ai đánh giá hiển thị "chưa đánh giá", không bao giờ là xanh. Một mức đánh giá hổ phách-đỏ hoặc đỏ sẽ mở một ngoại lệ với đồng hồ 48 giờ (có thể cấu hình); đồng hồ bị lỡ sẽ được nêu trong gói báo cáo.
- **Các quyết định để lại dấu vết.** Mọi thay đổi đều được kiểm toán, không thể sửa nhật ký kiểm toán thông qua dịch vụ, và người đưa ra một yêu cầu thay đổi không thể phê duyệt nó.

## Những gì phần mềm chưa mang lại cho bạn

- Gói báo cáo **chưa** bao gồm các quyết định tại cổng, cấp vốn và tiền theo năm tài chính, biểu đồ sốt vùng đệm, giá trị thu được, lợi ích đã được xác nhận hay bản đồ nhiệt năng lực. Chúng được liệt kê là `DP-5` đến `DP-10` trong [`tasks.md`](../../../../tasks.md), và gói báo cáo nêu rõ những phần nào chưa được xây dựng thay vì để trống.
- Chưa có PDF hay màn hình cho gói báo cáo: gói được cung cấp dưới dạng dữ liệu (`DP-11`).
- Phần mềm **chưa được ai chứng nhận hay kiểm toán**. Trang [con đường đến vận hành](../../path-to-production/index.md) nêu từng hạng mục cần có để đưa phần mềm vào sử dụng thực tế, kèm ước tính cho mỗi hạng mục.
- Một người bảo trì, không có thỏa thuận mức dịch vụ ([quản trị](../../../../GOVERNANCE.md)).

## Những quyết định thuộc về bạn

1. Có nên chạy thử nghiệm hay không, và trên phần nào của danh mục.
2. Ai là **nhà tài trợ** cho quy tắc ngoại lệ 48 giờ và ai được phát hành gói báo cáo hằng tháng.
3. Có nên cấp vốn để lấp các khoảng trống nêu trên (đánh giá, kiểm thử xâm nhập, thỏa thuận hỗ trợ) hay chấp nhận chúng.

## Đọc tiếp

[Con đường đến vận hành](../../path-to-production/index.md) · [Lộ trình](../../../../spec/15-roadmap.md) · [Các câu hỏi còn mở](../../../../spec/16-open-questions.md)
