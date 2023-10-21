FROM gcr.io/oss-fuzz-base/base-builder-rust:v1@sha256:17724a604288d0096cc3efdde4162692528d3abc430bb17ced45d74d838e536d
RUN apt-get update && apt-get install -y make autoconf automake libtool
COPY . $SRC/rsonpath
WORKDIR $SRC/rsonpath
COPY .clusterfuzzlite/build.sh $SRC/
