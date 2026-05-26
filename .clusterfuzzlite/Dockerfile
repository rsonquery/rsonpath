FROM gcr.io/oss-fuzz-base/base-builder-rust:v1@sha256:c19039c2c093ddac3472fa28880fe5d66118e9104694943408608aa029dc1d8a
RUN apt-get update && apt-get install -y make autoconf automake libtool
COPY . $SRC/rsonpath
WORKDIR $SRC/rsonpath
COPY .clusterfuzzlite/build.sh $SRC/