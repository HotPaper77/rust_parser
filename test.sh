#docker build . -t rust_parser
docker run  --memory="20m" --cpuset-cpus="0-1" -v "$PWD/samples/sample_1":/app/samples/sample_1  rust_parser