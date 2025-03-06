build:
	@rustup target add x86_64-unknown-linux-musl
	@cargo build --release --target x86_64-unknown-linux-musl

run: build
	./target/x86_64-unknown-linux-musl/release/httpd

clear:
	@cargo clean

docker:
	@docker build -t sadagatasgarov/myclipstash:0.0.1 .
	@docker push sadagatasgarov/myclipstash:0.0.1

docker-run:
	@docker rm clip
	@docker run -p8000:8000 --name clip sadagatasgarov/myclipstash:0.0.1

git:
	@git add .
	@git commit -m"Docker ficple yaradildi"
	@git push -u origin main