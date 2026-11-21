# Kiểm thử bảo mật độc lập

*Dành cho bên mua, người đánh giá hoặc người kiểm thử: việc kiểm thử bảo mật độc lập đối với phần mềm này nên bao quát những gì, dự án cung cấp gì để hỗ trợ, những gì đã làm và chưa làm, và những gì dành cho bạn. Đây là hướng dẫn, không phải báo cáo kiểm thử và không phải tư vấn pháp lý.*

## Hiện trạng

**Chưa có kiểm thử bảo mật độc lập nào được thực hiện.** Phần mềm này chưa có kiểm thử xâm nhập, kiểm toán của bên thứ ba, rà soát mã độc lập hay chứng nhận nào, và dự án không tuyên bố có ([con đường đến vận hành](../../path-to-production/index.md), [SECURITY](../../../../SECURITY.md)). Một lần quét thụ động chạy trong tích hợp liên tục của dự án (các quy tắc trong `.zap/rules.tsv`), và các bài kiểm thử riêng của dự án kiểm tra các biện pháp kiểm soát được liệt kê dưới đây; không cái nào thay thế được một người kiểm thử không phải là người viết phần mềm.

Đây là mục 1 và mục 9 của [danh sách kiểm tra bảo đảm](../../../../governance/assurance-checklist.md). Việc này thuộc về bên triển khai, cùng một người kiểm thử được chứng nhận, nhắm vào lần triển khai của **bạn**: dự án không thể kiểm thử hạ tầng lưu trữ, nhà cung cấp danh tính, mạng hay khóa của bạn.

## Cần kiểm thử gì

Hãy kiểm thử lần triển khai đúng như cách nó sẽ chạy, với đăng nhập được thực thi và nhà cung cấp danh tính của bạn đứng phía trước. Phạm vi đề xuất, dựa trên các biện pháp kiểm soát mà phần mềm tuyên bố:

| Mảng | Phần mềm tuyên bố gì | Người kiểm thử nên thử gì |
|---|---|---|
| Xác thực | Đăng nhập được bật theo mặc định; dịch vụ không khởi động nếu thiếu khóa xác minh hoặc, ở môi trường vận hành, khi đăng nhập bị tắt; token được xác minh ngoại tuyến dựa trên các khóa đã công bố | Token giả mạo, hết hạn, bị phát lại và ký sai; sai bên phát hành hoặc đối tượng; xoay vòng khóa; thiếu bộ khóa |
| Ủy quyền | Chính sách theo thuộc tính; các quy tắc ở mức bản ghi | Leo thang đặc quyền theo chiều ngang và chiều dọc trên mọi tuyến; thay đổi mã định danh trong đường dẫn và nội dung; các thuộc tính mà người gọi không được phép thêm |
| Kế hoạch riêng tư và dấu xử lý | Một kế hoạch riêng tư, và một kế hoạch trên mức được phép của người gọi, không nằm trong bất kỳ danh sách, số đếm, tổng số, kết quả tìm kiếm, nguồn cấp hay bản xuất nào đối với người gọi đó, kể cả quản trị viên | Mọi tuyến làm lộ sự tồn tại, tên, số đếm hay tổng số của một kế hoạch mà người gọi không đọc được; thông báo lỗi và thời gian phản hồi khác nhau giữa "bị ẩn" và "không tồn tại" |
| Truy cập khẩn cấp | Chỉ người đủ điều kiện, lý do nêu rõ, tối đa bốn giờ, một người thứ hai được báo | Sử dụng khi không đủ điều kiện; gia hạn một quyền; hành động mà người thứ hai không được báo |
| Bản ghi đóng băng và chỉ thêm vào | Nhật ký kiểm toán, các gói báo cáo ban giám đốc đã phát hành, lịch sử mức độ tin cậy về tiến độ, lịch sử trạng thái chương trình và vùng đệm dự án từ chối thay đổi và xóa trong cơ sở dữ liệu | Cập nhật hoặc xóa qua API, qua nhập hàng loạt, và qua bất kỳ tuyến nào chạm đến cùng các dòng đó |
| Phân tách nhiệm vụ | Người yêu cầu không thể phê duyệt yêu cầu thay đổi của chính mình | Phê duyệt với tư cách người yêu cầu, với tư cách quản trị viên, và bằng cách đổi người phê duyệt sau khi gửi |
| Tải lên và tệp đính kèm | Nội dung phải khớp với loại đã khai báo; chương trình bị từ chối; một điểm nối quét vi-rút tùy chọn sẽ đóng khi lỗi | Tệp đa dạng thức (polyglot), chương trình bị đổi tên, tệp quá lớn và sai định dạng, thủ thuật đường dẫn trong tên, và liệu bộ quét của bạn có thực sự được gọi hay không |
| Yêu cầu gửi đi | Các địa chỉ đích do người dùng cung cấp bị giới hạn ở các địa chỉ `https` công khai | Giả mạo yêu cầu phía máy chủ (SSRF) tới các địa chỉ riêng, địa chỉ link-local và địa chỉ siêu dữ liệu, chuyển hướng, DNS rebinding |
| Phiên và trình duyệt | Thời gian chờ khi không hoạt động, giới hạn cứng, cookie `httpOnly`, Content Security Policy nghiêm ngặt, danh sách cho phép CORS | Cố định và dùng lại phiên, giả mạo yêu cầu xuyên trang (CSRF), chèn mã vào giao diện, clickjacking |
| Giới hạn tần suất | Giới hạn theo từng người gọi | Vượt qua bằng cách đổi danh tính, tiêu đề hay đường dẫn; các giới hạn có còn hiệu lực khi qua proxy của bạn hay không |
| Công cụ dành cho chủ thể dữ liệu | Xuất và ẩn danh hóa bằng bí danh, chỉ quản trị viên, xem trước trước | Sử dụng bởi người không phải quản trị viên; tính đầy đủ so với [bản kê dữ liệu cá nhân](../../../../governance/personal-data-inventory.csv) |
| Sao lưu | Mã hóa tùy chọn, khôi phục đã được kiểm thử | Truy cập vào tệp sao lưu và khóa; khôi phục vào một môi trường sạch |

## Dự án cung cấp gì

- Mã nguồn, đặc tả và các bài kiểm thử, để việc rà soát mã có thể bắt đầu từ các tuyên bố.
- Một mô tả API đọc được bằng máy (`/api-docs/openapi.json`), với lược đồ yêu cầu và phản hồi cho công việc về gói báo cáo ban giám đốc; danh sách các tuyến được các bài kiểm thử đối chiếu với mã nguồn.
- Một cấu hình dùng một lần với đăng nhập tắt (`dev-insecure`, xem [README](../../../../README.md)) **chỉ cho máy của chính người kiểm thử**, và một kịch bản tạo bộ khóa kiểm thử (`scripts/smoke-keys.sh`) để có thể tạo token với các thuộc tính được chọn trên một lần triển khai kiểm thử.
- [Sơ đồ luồng dữ liệu](../../../../governance/data-flow.md), [sổ tay vận hành](../../../../operations/runbook.md) và [mục tiêu khôi phục](../../../../operations/recovery-objectives.md).
- Một cách riêng tư để báo cáo những gì tìm thấy: [SECURITY.md](../../../../SECURITY.md).

## Quy tắc tham gia

Hãy thống nhất các quy tắc này bằng văn bản trước mọi cuộc kiểm thử; dự án không thể đặt chúng cho môi trường của bạn.

- **Môi trường.** Kiểm thử trên một bản sao với dữ liệu thực tế nhưng không phải dữ liệu thật. Không kiểm thử môi trường vận hành trừ khi bạn đã quyết định như vậy bằng văn bản.
- **Ủy quyền.** Người kiểm thử có sự cho phép bằng văn bản của bạn, một đầu mối liên hệ có tên, và một khung thời gian nêu rõ.
- **Ngoài phạm vi.** Tấn công từ chối dịch vụ bằng lưu lượng, kỹ nghệ xã hội nhắm vào nhân viên, và các dịch vụ bên thứ ba (nhà cung cấp danh tính, nơi lưu trữ của bạn) nằm ngoài phạm vi trừ khi chủ sở hữu của từng cái đồng ý. Phạm vi riêng của dự án nằm trong [SECURITY](../../../../SECURITY.md).
- **Dữ liệu.** Dữ liệu cá nhân thật không được dùng, không được sao chép ra ngoài và không được đưa vào báo cáo.
- **Khóa.** Khóa kiểm thử tách biệt với khóa vận hành và bị hủy sau đó.

## Báo cáo và kiểm thử lại

- Các phát hiện về phần mềm này được gửi riêng cho người bảo trì, như [SECURITY](../../../../SECURITY.md) nêu, kèm cách tái hiện, phiên bản hoặc commit, và tác động. Mục tiêu của dự án là xác nhận trong 5 ngày làm việc và đưa ra kế hoạch trong 15 ngày; đó là mục tiêu, không phải bảo đảm. Chỉ có một người bảo trì và không có thỏa thuận mức dịch vụ.
- Các phát hiện về lần triển khai của bạn được gửi cho người phụ trách của chính bạn.
- Hãy yêu cầu người kiểm thử xếp hạng từng phát hiện, nêu những gì **chưa** được kiểm thử, và kiểm thử lại các bản sửa. Một báo cáo chỉ liệt kê những gì tìm thấy không cho bạn biết những gì đã bị bỏ sót.
- Hãy giữ báo cáo, phạm vi, ngày tháng và lần kiểm thử lại trong hồ sơ của chính bạn. Dự án không thể giữ chúng thay bạn.

## Giới hạn của tài liệu này

Tài liệu này liệt kê các mảng cần bao quát, không phải một kế hoạch kiểm thử đầy đủ, và các tuyên bố trong bảng là của chính dự án. Nó không chứng nhận điều gì, và một cuộc kiểm thử sạch chỉ là bằng chứng về một lần triển khai tại một thời điểm.

## Những quyết định thuộc về bạn

Ai kiểm thử, khi nào, nhắm vào đâu, với dữ liệu nào và theo quy tắc nào; các phát hiện được xếp hạng ra sao và phải được sửa trước thời điểm nào; và kết quả có được chia sẻ với những người đánh giá của bạn hay không.

## Đọc tiếp

[Danh sách kiểm tra bảo đảm](../../../../governance/assurance-checklist.md) · [Con đường đến vận hành](../../path-to-production/index.md) · [Giám đốc an ninh](../chief-security-officer/index.md) · [Quản trị, rủi ro, tuân thủ](../governance-risk-compliance/index.md)
