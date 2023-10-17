FROM gcr.io/oss-fuzz-base/base-builder-rust:v1@sha256:64cc49837d24653d77849ec7e88f48cc0e5090fdc202eadf60c3067ca22d3f5e
RUN apt-get update && apt-get install -y make autoconf automake libtool
COPY . $SRC/rsonpath
WORKDIR $SRC/rsonpath
COPY .clusterfuzzlite/build.sh $SRC/
