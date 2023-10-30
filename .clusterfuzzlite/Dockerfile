FROM gcr.io/oss-fuzz-base/base-builder-rust:v1@sha256:ac9054c9c1b969aeb40d26b9bd2a792c2d1dc23f54954d271ddb62c4cc1c8e29
RUN apt-get update && apt-get install -y make autoconf automake libtool
COPY . $SRC/rsonpath
WORKDIR $SRC/rsonpath
COPY .clusterfuzzlite/build.sh $SRC/
