FROM gcr.io/oss-fuzz-base/base-builder-rust:v1@sha256:52508f7d366f93c0523297783183ae24307567debdc779ea142517e0132601b9
RUN apt-get update && apt-get install -y make autoconf automake libtool
COPY . $SRC/rsonpath
WORKDIR $SRC/rsonpath
COPY .clusterfuzzlite/build.sh $SRC/
