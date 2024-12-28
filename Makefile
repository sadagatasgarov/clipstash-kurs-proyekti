build:
	cargo build --release

run: build
	./target/release/httpd

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