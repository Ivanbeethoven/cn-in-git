# 构建阶段
FROM rust:1.85 AS builder

# 安装必要依赖
RUN apt-get update && \
    apt-get install -y musl-tools && \
    rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY . .

# 以静态链接的方式构建应用，确保最终二进制文件不依赖外部库
RUN cargo build --target x86_64-unknown-linux-musl --release

# 最终运行镜像
FROM alpine

# 将构建好的二进制文件从 builder 阶段复制到最终镜像
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/cn-in-git /bin/cig

# 设置容器启动时执行的命令
CMD ["sh", "-c", "tail -f /dev/null"]