FROM gcr.io/oss-fuzz-base/base-builder-rust:v1@sha256:c9a3eb6764f7fc4da8bec126cced774d8f8abbf3c828e935ec33b86001da2a4c
RUN apt-get update && apt-get install -y make autoconf automake libtool
COPY . $SRC/rsonpath
WORKDIR $SRC/rsonpath
COPY .clusterfuzzlite/build.sh $SRC/
