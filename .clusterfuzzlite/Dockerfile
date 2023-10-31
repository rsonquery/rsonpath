FROM gcr.io/oss-fuzz-base/base-builder-rust:v1@sha256:7c5c509f4a9e3770174f58d3af3f5bda8904098ee10569cff90796d5cf78151b
RUN apt-get update && apt-get install -y make autoconf automake libtool
COPY . $SRC/rsonpath
WORKDIR $SRC/rsonpath
COPY .clusterfuzzlite/build.sh $SRC/
