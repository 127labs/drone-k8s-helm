FROM scratch

ADD ./target/release/yakp /

CMD ["/yakp"]
