FROM gcr.io/oss-fuzz-base/base-builder-rust:v1@sha256:b8d7dcd1038021b4338a150132b9771f78d64a6261282116b91b95710fbde84f
RUN apt-get update && apt-get install -y make autoconf automake libtool
COPY . $SRC/rsonpath
WORKDIR $SRC/rsonpath
COPY .clusterfuzzlite/build.sh $SRC/