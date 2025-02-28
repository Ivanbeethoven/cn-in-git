# 构建阶段
FROM rust:1.70 as builder

# 安装必要依赖
RUN apt-get update && \
    apt-get install -y musl-dev pkg-config && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

# 构建应用
RUN cargo build --release

# 最终运行镜像
FROM alpine:latest

# 安装运行时依赖
RUN apk add --no-cache libgcc

WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/cig /app/cig

# 设置执行权限
RUN chmod +x /app/cig

# 设置入口点
ENTRYPOINT ["/app/cig"]
