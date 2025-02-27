#!/usr/bin/env bash
# Oliver Epper <oliver.epper@gmail.com>

source "$(dirname "$0")"/functions.sh
source "$(dirname "$0")"/configure-pjproject-base.sh

if [ $# -ne 2 ]; then
	print_error "Usage: $0 <build-dir> <prefix>"
	exit 1
fi

BUILD_DIR=$1
PREFIX=$2

if [ ! -d "$PREFIX" ]; then
	print_error "The directory \"${PREFIX}\" does not exist. Please build Opus, first."
	exit 1
fi

pushd "${BUILD_DIR}" || exit 1

cat << EOF > user.mak
export CFLAGS += -Wno-unused-label
export LDFLAGS += -framework Network -framework Security
EOF

cat << EOF > pjlib/include/pj/config_site.h
#include <pj/config_site_sample.h>
#define PJ_HAS_SSL_SOCK 1
#undef PJ_SSL_SOCK_IMP
#define PJ_SSL_SOCK_IMP PJ_SSL_SOCK_IMP_APPLE
EOF

# Codecs
PARAMS=(--disable-opencore-amr --disable-bcg729 --enable-g711-codec \
	--enable-g722-codec --disable-g7221-codec --disable-gsm-codec --without-external-gsm \
	--disable-ilbc-codec --disable-l16-codec --disable-silk --disable-speex-codec \
	--without-external-speex --disable-speex-aec)

# other settings
PARAMS+=(--disable-ffmpeg --enable-pjsua2 --enable-sound --enable-libsrtp \
	--disable-upnp --disable-v4l2 --disable-video --disable-vpx --disable-libyuv)

# Opus Codec
PARAMS+=(--with-opus="${PREFIX}")

SDKPATH=$(xcrun -sdk macosx --show-sdk-path)
PJ_SDK_NAME=$(basename "${SDKPATH}")
export CFLAGS="${CFLAGS} -fsanitize=address -isysroot $SDKPATH -mmacosx-version-min=11 -DPJ_SDK_NAME=\"\\\"${PJ_SDK_NAME}\\\"\""
export CXXFLAGS="${CXXFLAGS} -fsanitize=address"
export LDFLAGS="-fsanitize=address -isysroot ${SDKPATH}"
./aconfigure --prefix= --host=arm-apple-darwin "${PARAMS[@]}"

popd || exit 1
