FROM gcr.io/oss-fuzz-base/base-builder-rust:v1@sha256:d12105170b12f46f8926d2557e64576d8cc994dddbf38321dc587569bec174b8
RUN apt-get update && apt-get install -y make autoconf automake libtool
COPY . $SRC/rsonpath
WORKDIR $SRC/rsonpath
COPY .clusterfuzzlite/build.sh $SRC/