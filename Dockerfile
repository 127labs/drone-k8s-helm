FROM scratch

ADD ./target/release/yakp /bin/

ENTRYPOINT ["/bin/yakp"]
