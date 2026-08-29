FROM gcr.io/oss-fuzz-base/base-builder-rust:v1@sha256:35767be864a85d01496cda483b869e50f1e62730e27e6ce37ff2304aa1242be1
RUN apt-get update && apt-get install -y make autoconf automake libtool
COPY . $SRC/rsonpath
WORKDIR $SRC/rsonpath
COPY .clusterfuzzlite/build.sh $SRC/