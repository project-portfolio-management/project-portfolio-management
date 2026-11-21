# Dành cho Giám đốc công nghệ

*Giám đốc công nghệ (CTO): kiến trúc, chất lượng, cách mở rộng, và những chỗ kỹ thuật còn mỏng.*

## Kiến trúc

Ba dự án con, một thực thể ([kiến trúc](../../../../spec/08-architecture.md)):

| Thành phần | Công nghệ |
|---|---|
| Thư viện đối chiếu | Rust; đối chiếu theo cặp có giải thích được, mang tính xác định và xác suất; được dịch vụ nhúng vào |
| Dịch vụ | Rust, axa trên axum, sqlx, PostgreSQL 18; REST dưới `/api`, OpenAPI được suy ra từ các tuyến |
| Giao diện | SvelteKit 3, SPA Svelte 5 với một backend-for-frontend; các danh mục i18n |

Bộ công cụ được cố định trong [`rust-toolchain.toml`](../../../../rust-toolchain.toml). Lưu trữ dữ liệu nằm trong [spec 10](../../../../spec/10-persistence.md); API nằm trong [spec 09](../../../../spec/09-api-surface.md).

## Những điểm mạnh kỹ thuật đáng biết

- **Các bất biến nằm trong cơ sở dữ liệu ở nơi chúng quan trọng:** nhật ký kiểm toán từ chối thay đổi và xóa; các gói báo cáo ban giám đốc đã phát hành và các đánh giá mức độ tin cậy về tiến độ là bất biến hoặc chỉ thêm vào được nhờ trigger; các ràng buộc CHECK hỗ trợ các quy tắc của dịch vụ.
- **Kiểm thử bảo vệ:** các kiểm thử điều tra (census) làm bản dựng thất bại khi một tuyến tổng hợp mới bỏ qua bộ lọc kế hoạch riêng tư, một tuyến danh sách không phân trang, một cột dữ liệu cá nhân thiếu trong bản kê, hoặc một tuyến thiếu trong OpenAPI. Chúng được sửa bằng cách phân loại điều mới, không phải bằng cách làm yếu bài kiểm thử.
- **Kiểm thử với token thật:** việc ủy quyền được kiểm thử từ đầu đến cuối với các token PASETO đã ký (mỗi bài kiểm thử trong một tiến trình riêng, vì các thiết lập xác thực chỉ được đọc một lần cho mỗi tiến trình).
- **Chuỗi cung ứng:** `cargo deny`, bảng kê vật liệu phần mềm, nguồn gốc bản dựng khi phát hành, một chính sách bảo mật.
- **CI** xây dựng và kiểm thử mọi thứ ở mỗi lần đẩy mã; xem `.github/workflows/`.

## Những chỗ còn mỏng

- Một người bảo trì, nên hệ số xe buýt (bus factor) là một ([`MAINTAINERS.md`](../../../../MAINTAINERS.md)).
- Hiệu năng mới biết đại khái; bộ nhớ đệm phản hồi chỉ bao phủ tuyến đường găng; các kế hoạch rất lớn chạy chậm.
- Phiên nằm trong bộ nhớ trừ khi cấu hình Redis.
- Một số phần của gói báo cáo ban giám đốc và mọi màn hình của gói báo cáo chưa được xây dựng ([`tasks.md`](../../../../tasks.md)).
- Các công cụ AI được dùng để xây dựng nó; sản phẩm không chứa mô hình nào. Xem [tuyên bố về AI](../../../../AI_STATEMENT.md).

## Những quyết định thuộc về bạn

Cấu trúc lưu trữ và Redis, có nên tách nhánh (fork) hay theo sát bản gốc, ai rà soát mã một cách độc lập, và dịch vụ khớp với nhà cung cấp danh tính của bạn như thế nào (bộ khóa PASETO, hoặc OIDC qua Keycloak hay Entra).

## Đọc tiếp

[Kiến trúc](../../../../spec/08-architecture.md) · [Chiến lược kiểm thử](../../../../spec/11-testing-strategy.md) · [Tình trạng triển khai](../../../../spec/14-implementation-status.md) · [Đo hiệu năng](../../../../BENCHMARKS.md)
