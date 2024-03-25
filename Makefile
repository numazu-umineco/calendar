.PHONY: setup setup/container setup/storage setup/database

setup: setup/container setup/storage setup/database

setup/container:
	docker-compose build
	docker-compose run --rm admin npm install

setup/storage:
	docker-compose up -d storage
	docker-compose run --rm storage_ctl /storage-setup.sh

setup/database:
	docker-compose run --rm api bundle exec rails db:create db:migrate db:seed
