gen_spec:
	python gen_mlflow_openapi.py

gen_client:
	docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate -i /local/mlflow.yaml -g rust --package-name mlflow-client -o /local/out/rust

run_swagger:
	docker run -d -p 8080:8080 swaggerapi/swagger-editor
