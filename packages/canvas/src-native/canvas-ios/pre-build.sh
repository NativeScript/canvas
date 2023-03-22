#!/usr/bin/env bash
set -e

source $HOME/.cargo/env
source $HOME/.profile

whereis cc

echo $PATH

function to_bool() {
  local arg="$1"
  case "$(echo "$arg" | tr '[:upper:]' '[:lower:]')" in
  [0-9]+)
    if [ $arg -eq 0 ]; then
      echo false
    else
      echo true
    fi
    ;;
  n | no | f | false) echo false ;;
  y | yes | t | true) echo true ;;
  *)
    if [ -n "$arg" ]; then
      echo "warning: invalid boolean argument ('$arg'). Expected true or false" >&2
    fi
    echo false
    ;;
  esac
}

pushd "$SRCROOT/../canvas-native"

IS_SIMULATOR=false

IS_RELEASE=false
RUST_BUILD_TYPE=""
RUST_BUILD_TARGET=""

echo $CONFIGURATION

if [[ $CONFIGURATION == Release ]]; then
  IS_RELEASE=true
fi



if $IS_RELEASE; then
  RUST_BUILD_TYPE="--release"
fi



PLATFORM_NAME="${PLATFORM_NAME:-iphoneos}"
CURRENT_ARCH="${CURRENT_ARCH}"

if [ -z "$CURRENT_ARCH" ] || [ "$CURRENT_ARCH" == "undefined_arch" ]; then
    # Xcode 10 beta sets CURRENT_ARCH to "undefined_arch", this leads to incorrect linker arg.
    # it's better to rely on platform name as fallback because architecture differs between simulator and device

    if [[ "$PLATFORM_NAME" == *"simulator"* ]]; then
        CURRENT_ARCH="arm64"
        IS_SIMULATOR=true
    else
        CURRENT_ARCH="arm64"
    fi
fi


if [[ $CURRENT_ARCH == x86_64 ]]; then
  RUST_BUILD_TARGET="x86_64-apple-ios"
fi


if [[ $CURRENT_ARCH == arm64 ]]; then
  if [[ $IS_SIMULATOR == false ]]; then
    RUST_BUILD_TARGET="aarch64-apple-ios"
  else
    RUST_BUILD_TARGET="aarch64-apple-ios-sim"
  fi
fi


cargo +nightly build -Z build-std='std' --manifest-path Cargo.toml --target $RUST_BUILD_TARGET $RUST_BUILD_TYPE -p canvas-ios
popd
