
```
curl localhost:8482/predict -H 'content-type: application/json' -d '{"model_name":"tensorflower","model_version":"2"}'

docker run -d -p16686:16686 -p4317:4317 -e COLLECTOR_OTLP_ENABLED=true jaegertracing/all-in-one:latest
```