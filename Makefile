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
	cd web && docker build -t layer5/image-hub-web:latest .

build-api:
	cd api && docker build -t layer5/image-hub-api:latest .

build-envoy:
	cd rate-limit-filter && docker build -t layer5/image-hub-envoy:latest .

dev-run-api: build-api deploy

dev-run-web: 
	cd web && yarn serve

images-push:
	docker push layer5/image-hub-web:latest
	docker push layer5/image-hub-api:latest
	docker push layer5/image-hub-envoy:latest