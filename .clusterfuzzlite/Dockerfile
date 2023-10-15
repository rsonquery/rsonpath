FROM gcr.io/oss-fuzz-base/base-builder-rust:v1@sha256:c53b58ef6497d118c98cf920a63db950fcab182ea0c58ad6d7166dbcfdcacd31
RUN apt-get update && apt-get install -y make autoconf automake libtool
COPY . $SRC/rsonpath
WORKDIR $SRC/rsonpath
COPY .clusterfuzzlite/build.sh $SRC/
