build:
	cd rate-limit-filter && cargo +nightly build --target=wasm32-unknown-unknown --release 

deploy:
	docker-compose up --build --remove-orphans

# shows only the logs related to WASM filter/singleton 
deploy-filtered:
	docker-compose up --build --remove-orphans | grep "\[wasm\]\|Starting"

run: build deploy

run-filtered: build deploy-filtered

clean:
	cargo clean

build-web:
	cd web && docker build -t layer5io/dockercon-2020-web:dev .

build-api:
	cd api && docker build -t layer5io/dockercon-2020-api:dev .

dev-run-api: build-api deploy

dev-run-web: 
	cd web && yarn serve
	
	