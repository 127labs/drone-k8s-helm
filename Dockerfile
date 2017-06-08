FROM alpine:latest

ENV VERSION v2.4.2
ENV FILENAME helm-${VERSION}-linux-amd64.tar.gz
ENV KUBECTL v1.6.4

RUN apk -Uuv add curl bash && rm /var/cache/apk/*

ADD http://storage.googleapis.com/kubernetes-helm/${FILENAME} /tmp

ADD https://storage.googleapis.com/kubernetes-release/release/${KUBECTL}/bin/linux/amd64/kubectl /tmp

RUN tar -zxvf /tmp/${FILENAME} -C /tmp \
  && mv /tmp/linux-amd64/helm /bin/helm \
  && chmod +x /tmp/kubectl \
  && mv /tmp/kubectl /bin/kubectl \
  && rm -rf /tmp

ADD ./target/release/drone-k8s-helm /bin/drone-k8s-helm

ENTRYPOINT ["/bin/drone-k8s-helm"]
