.PHONY: setup

setup:
	docker-compose build
	docker-compose run --rm admin npm install
	docker-compose up -d storage
	docker run --rm --network calendar --env-file ${CURDIR}/.env -v $(CURDIR)/storage-setup.sh:/storage-setup.sh --entrypoint "/storage-setup.sh" minio/mc:latest
	docker-compose run --rm api bundle exec rails db:create db:migrate db:seed
