# Opentelemetry Initializer

This initializer is responsible for initializing tracing with opentelemetry and adding it as a middleware in axum. 

Because opentelemetry tracing initializer is not compatible with loco's default tracing initialization we must turn off the default logging.

Follow the below steps in order to enable initializer:

1) Add opentelemetry initializer to initializers list 
```
async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
    Ok(vec![
        Box::new(loco_extras::initializers::opentelemetry::OpenTelemetryInitializer)
    ])
}
```

2) Add function that disables initialization of logger 
````
fn init_logger(_config: &config::Config, _env: &Environment) -> Result<bool> {
    Ok(true)
}
````

# Local development:

You can run jaeger locally via docker for development purposes. 
```
docker run --rm --name jaeger -d -p 5778:5778 -p 16686:16686 -p 4317:4317 -p 4318:4318 -p 14250:14250 -p 14268:14268 -p 9411:9411 jaegertracing/jaeger:2.0.0 --set receivers.otlp.protocols.http.endpoint=0.0.0.0:4318 --set receivers.otlp.protocols.grpc.endpoint=0.0.0.0:4317
```
You can find more information on jaeger's [documentation](https://www.jaegertracing.io/docs/2.0/getting-started/)

Reference [OpenTelemetry SDK configuration](https://opentelemetry.io/docs/concepts/sdk-configuration/general-sdk-configuration/)
for configuring opentelemetry via environmental variables