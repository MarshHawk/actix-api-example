from opentelemetry import trace
from opentelemetry.exporter.otlp.proto.grpc.trace_exporter import OTLPSpanExporter
from opentelemetry.sdk.resources import Resource
from opentelemetry.sdk.trace import TracerProvider
from opentelemetry.sdk.trace.export import BatchSpanProcessor

# Set the tracer provider
trace.set_tracer_provider(
    TracerProvider(
        resource=Resource.create({"service.name": "my_service"})
    )
)

# Set the exporter
otlp_exporter = OTLPSpanExporter(
    endpoint="localhost:4317",  # Update this to the appropriate endpoint
    insecure=True,
)

# Set the span processor
trace.get_tracer_provider().add_span_processor(
    BatchSpanProcessor(otlp_exporter)
)

# Get a tracer
tracer = trace.get_tracer(__name__)

# Create a span
with tracer.start_as_current_span("my_span"):
    print("Hello, world!")