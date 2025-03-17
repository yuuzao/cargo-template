
# 使用git commit id作为tag
TAG := $(shell git rev-parse --short HEAD)

all: build tag push

build:
    #如果docker images中tag存在，则跳过
	if docker images | grep -q ${TAG}; then \
		echo "{{project-name}}:${TAG} already exists"; \
	else \
		docker build -t {{project-name}}:${TAG} .; \
	fi

tag: build
	docker tag {{project-name}}:${TAG} registry.rwx.cat/{{project-name}}:${TAG}
	docker tag {{project-name}}:${TAG} registry.rwx.cat/{{project-name}}:latest

push: tag
	docker push registry.rwx.cat/{{project-name}}:${TAG}
	docker push registry.rwx.cat/{{project-name}}:latest

compose:
	curl -X POST http://192.168.6.247:18080/compose/up -F "compose_name={{project-name}}" -F "data=@docker-compose.yml"