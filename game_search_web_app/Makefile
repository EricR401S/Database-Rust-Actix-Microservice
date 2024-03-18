install:
	cargo clean &&\
		cargo build -j 1

build:
	docker build -t vg_search_actix .

rundocker:
	# docker run -it --rm -p 8080:8080 vg_search_actix
	# docker run -dp 8080:8080 vg_search_actix


format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet


deploy-aws:
	cargo build
	aws ecr get-login-password --region us-east-2 | docker login --username AWS --password-stdin 667719398048.dkr.ecr.us-east-2.amazonaws.com	
	docker build -t vg_search_aws_actix .	
	docker tag vg_search_aws_actix:latest 667719398048.dkr.ecr.us-east-2.amazonaws.com/vg_search_aws_actix:latest
	docker push 667719398048.dkr.ecr.us-east-2.amazonaws.com/vg_search_aws_actix:latest
run:
	cargo run 

all: format lint test run