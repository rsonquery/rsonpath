FROM gcr.io/oss-fuzz-base/base-builder-rust:v1@sha256:73fe864d1d31009281fe35c1b0183d0ba8160b64908ab68989421cddb7c2e729
RUN apt-get update && apt-get install -y make autoconf automake libtool
COPY . $SRC/rsonpath
WORKDIR $SRC/rsonpath
COPY .clusterfuzzlite/build.sh $SRC/
