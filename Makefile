
# 使用git commit id作为tag
TAG := $(shell git rev-parse --short HEAD)

all: build tag push

build:
    #如果docker images中tag存在，则跳过
	if docker images | grep -q ${TAG}; then \
		echo "template:${TAG} already exists"; \
	else \
		docker build -t template:${TAG} .; \
	fi

tag: build
	docker tag template:${TAG} registry.rwx.cat/template:${TAG}
	docker tag template:${TAG} registry.rwx.cat/template:latest

push: tag
	docker push registry.rwx.cat/template:${TAG}
	docker push registry.rwx.cat/template:latest

compose:
	curl -X POST http://192.168.6.247:18080/compose/up -F "compose_name=template" -F "data=@docker-compose.yml"