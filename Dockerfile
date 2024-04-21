FROM rust:1.77-bookworm


ENV ZIGLANG_INSTALL_FILE "zig-linux-x86_64-0.12.0.tar.xz"
ENV ZIGLANG_DOWNLOAD_URL "https://ziglang.org/download/0.12.0/$ZIGLANG_INSTALL_FILE"


WORKDIR /work

# zigbuildをセットアップする
# 後方互換性を期待して古いglibcで動作するバイナリをビルドすることができるようになる
# これは静的リンクだと問題があるとき、動的リンクのまま可搬性を高める手段のひとつ
# cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.17
RUN wget $ZIGLANG_DOWNLOAD_URL && \
    mkdir /usr/local/zig && \
    tar Jxvf $ZIGLANG_INSTALL_FILE -C /usr/local/zig --strip-components 1 && \
    echo 'export PATH=$PATH:/usr/local/zig' >> /root/.bashrc && \
    cargo install cargo-zigbuild

# cargoのコンポーネントを追加する
RUN rustup component add rustfmt
