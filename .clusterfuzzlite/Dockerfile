FROM gcr.io/oss-fuzz-base/base-builder-rust:v1@sha256:d5045a3387651e70bbfe64018a10540ae420d2213c48f962c5372f6d851d8b9a
RUN apt-get update && apt-get install -y make autoconf automake libtool
COPY . $SRC/rsonpath
WORKDIR $SRC/rsonpath
COPY .clusterfuzzlite/build.sh $SRC/