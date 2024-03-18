install:
	cd game_search_web_app\
		cargo clean &&\
			cargo build &&\
				cargo run 

build:
	docker build -t vg_search_actix .

rundocker:
	# docker run -it --rm -p 8080:8080 vg_search_actix
	docker run -dp 8080:8080 vg_search_actix


format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet


deploy-aws:
	aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin 667719398048.dkr.ecr.us-east-1.amazonaws.com
	docker build -t vg_search_aws .
	docker tag vg_search_aws:latest 667719398048.dkr.ecr.us-east-1.amazonaws.com/vg_search_aws:latest
	docker push 667719398048.dkr.ecr.us-east-1.amazonaws.com/vg_search_aws:latest

deploy-aws-from-root:
	cd game_search_web_app &&\
	cargo build --release
	aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin 667719398048.dkr.ecr.us-east-1.amazonaws.com
	docker build -t vg_search_aws .
	docker tag vg_search_aws:latest 667719398048.dkr.ecr.us-east-1.amazonaws.com/vg_search_aws:latest
	docker push 667719398048.dkr.ecr.us-east-1.amazonaws.com/vg_search_aws:latest

database-create:
	cd ./make_vg_db &&\
		cargo build &&\
			cargo run

local-run: install

local-docker-run: 
	cd game_search_web_app\
		docker build -t vg_search_actix . &&\
			docker run -it --rm -p 8080:8080 vg_search_actix
	
all: format lint test run