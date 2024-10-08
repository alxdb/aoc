FROM alpine:latest

RUN <<EOF
apk add --no-cache build-base cmake
EOF

RUN --mount=type=bind,source=.,target=/workarea --mount=type=cache,target=/build <<EOF
set -e
cmake -S /workarea -B /build
cmake --build /build
cp -r /build /out
EOF

CMD ["ctest", "--test-dir", "/out"]
