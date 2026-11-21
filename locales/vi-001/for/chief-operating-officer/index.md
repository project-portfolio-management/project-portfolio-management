# Dành cho Giám đốc vận hành

*Giám đốc vận hành (COO): những gì bạn điều hành hằng ngày, phần mềm làm gì cho việc triển khai, và việc vận hành nó đòi hỏi gì.*

## Phần mềm làm gì cho hoạt động vận hành

- **Lập kế hoạch và kiểm soát triển khai:** kế hoạch, nhiệm vụ, quan hệ phụ thuộc, tính toán đường găng, cột mốc xuyên các kế hoạch, sprint và giai đoạn, biểu đồ burndown và các chỉ số luồng.
- **Năng lực:** phân bổ, khả năng sẵn sàng, nghỉ phép và các khoảng thời gian không làm việc, vai trò nguồn lực, khối lượng công việc và mức sử dụng được suy ra. (Bản đồ nhiệt năng lực theo nhóm kỹ năng và theo tháng cho gói báo cáo ban giám đốc chưa được xây dựng: `DP-10`.)
- **Thời gian và chi phí:** bảng chấm công, thời gian đã ghi, đơn giá chi phí (giới hạn cho bộ phận tài chính), ngân sách và chi phí được suy ra.
- **Luồng quản trị:** sổ đăng ký vấn đề, yêu cầu thay đổi với phân tách nhiệm vụ (người yêu cầu không thể phê duyệt), phê duyệt, rà soát, các trường hợp nhân sự vào/chuyển/rời.
- **Ngoại lệ:** mức độ tin cậy về tiến độ ở mức đỏ hoặc hổ phách-đỏ mở một ngoại lệ cần có nguyên nhân, quyết định cần đưa ra, một người phụ trách có tên và hạn ra quyết định trong vòng 48 giờ (khoảng thời gian có thể cấu hình, từ 1 đến 720 giờ). Các vi phạm xuất hiện trong gói báo cáo ban giám đốc và trong `GET /api/exceptions`.

## Việc vận hành đòi hỏi gì

| Nhu cầu | Chi tiết |
|---|---|
| Dịch vụ | Một dịch vụ Rust, PostgreSQL 18, Redis tùy chọn (bộ nhớ đệm khi vận hành, phiên dùng chung tùy chọn), giao diện SvelteKit |
| Sổ tay vận hành | [gói vận hành](../../../../operations/README.md): triển khai, sổ tay vận hành, sao lưu và khôi phục, mục tiêu khôi phục, nâng cấp và di chuyển |
| Các tác vụ theo lịch | Không có gì tự lên lịch. Việc xóa theo thời hạn lưu giữ, thông báo vi phạm ngoại lệ và các tác vụ tương tự phải được người vận hành của bạn đặt vào một bộ lập lịch |
| Hỗ trợ | Một người bảo trì, không có SLA; hãy dự trù ngân sách cho hỗ trợ tuyến đầu của chính bạn |

## Biết các giới hạn

- Một kế hoạch có vài nghìn nhiệm vụ lập lịch chậm; câu trả lời đầy đủ bị từ chối khi trên 5.000 nhiệm vụ trừ khi được yêu cầu (`?full=true` hoặc `?only=critical`). Số liệu và lưu ý nằm trong [`BENCHMARKS.md`](../../../../BENCHMARKS.md); chúng chỉ là bậc độ lớn từ một máy, không phải năng lực.
- Có màn hình cho phần lớn không gian làm việc nhưng **chưa** có cho yêu cầu thay đổi, vấn đề, gói báo cáo hay mức độ tin cậy về tiến độ; hiện nay những phần đó chỉ dùng qua API.

## Những quyết định thuộc về bạn

Ai phụ trách từng ngoại lệ, ai phát hành gói báo cáo hằng tháng (nhịp độ và lời nhắc là `DP-12`, chưa xây dựng), và quy tắc ngoại lệ đưa vào chu kỳ rà soát nào.

## Đọc tiếp

[Gói vận hành](../../../../operations/README.md) · [Sổ tay vận hành](../../../../operations/runbook.md) · [Mục tiêu khôi phục](../../../../operations/recovery-objectives.md)
