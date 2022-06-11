#!/bin/sh

curl --request POST \
  --url http://localhost:7727/print-file \
  --header 'content-type: multipart/form-data' \
  --header 'content-type: multipart/form-data; boundary=---011000010111000001101001' \
  --form some-file=@Cargo.toml \
  --form some-string=something