FROM centos:centos7.9.2009 as builder
RUN mkdir -p /app/redis-admin/src/
RUN export RUSTUP_DIST_SERVER="https://rsproxy.cn" \
    && export RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup" \
    && curl --proto '=https' --tlsv1.2 -sSf https://rsproxy.cn/rustup-init.sh > /tmp/rustup-init.sh \
    && chmod +x /tmp/rustup-init.sh \
    && /tmp/rustup-init.sh -y \
    && yum install -y git gcc gcc-c++
ADD cargo_config /root/.cargo/config
ADD src/ /app/redis-admin/src/
ADD Cargo.toml /app/redis-admin/
RUN cd /app/redis-admin/ \
    && /root/.cargo/bin/cargo build --release

FROM centos:centos7.9.2009
RUN mkdir /app/ \
    && rm -f /etc/localtime \
    && ln -s /usr/share/zoneinfo/Asia/Shanghai /etc/localtime
ADD docker-entrypoint.sh /
ADD templates/ /app/templates/
ADD static/ /app/static/
RUN chmod +x /docker-entrypoint.sh 
ENV LC_ALL en_US.UTF-8
EXPOSE 8080
WORKDIR /app/
COPY --from=builder /app/redis-admin/target/release/redis-admin /app/
#ENTRYPOINT ["/docker-entrypoint.sh"]
CMD ["/bin/sh", "-c", "/docker-entrypoint.sh"]
