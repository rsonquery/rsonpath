FROM gcr.io/oss-fuzz-base/base-builder-rust:v1@sha256:e155522ceab10af0de09331bb56acf6e0534b95d290de5a8b97539260c3908a9
RUN apt-get update && apt-get install -y make autoconf automake libtool
COPY . $SRC/rsonpath
WORKDIR $SRC/rsonpath
COPY .clusterfuzzlite/build.sh $SRC/
