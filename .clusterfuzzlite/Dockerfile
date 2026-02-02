FROM gcr.io/oss-fuzz-base/base-builder-rust:v1@sha256:09b768c3c1f0054e8196042997025f5f1112b1382b70d62859c03113bd8c2438
RUN apt-get update && apt-get install -y make autoconf automake libtool
COPY . $SRC/rsonpath
WORKDIR $SRC/rsonpath
COPY .clusterfuzzlite/build.sh $SRC/