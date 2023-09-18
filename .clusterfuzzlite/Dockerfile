FROM gcr.io/oss-fuzz-base/base-builder-rust:v1@sha256:51593bf0c908e3d7912d4ea7cd89dc885882eaab0015a8a672f463957f1a9048
RUN apt-get update && apt-get install -y make autoconf automake libtool
COPY . $SRC/rsonpath
WORKDIR $SRC/rsonpath
COPY .clusterfuzzlite/build.sh $SRC/
