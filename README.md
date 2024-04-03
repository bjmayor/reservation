# Revervation
A core revservation service that resolves the problem of reserving a resource for a period of time.

# 环境
`cargo install cargo-deny`  防止安装未授权的依赖, git commit 钩子在用
`cargo install cargo-nextest` 替代rust的测试工具，功能更强大
`cargo install cargo-generate` 根据模板生成项目。 第15讲用到

直接cargo install sqlx 的是0.7.2版本。
但是sqlx_database_tester依赖的是0.6.3版本。
导致编译不过，指定0.6.3版本就好了。
