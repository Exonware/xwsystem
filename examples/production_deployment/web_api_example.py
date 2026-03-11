#!/usr/bin/env python3
#exonware/xwsystem/examples/production_deployment/web_api_example.py
"""
Production Web API Example with xwsystem
This example demonstrates a production-ready REST API using xwsystem
for serialization, security, and monitoring.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
"""

from fastapi import FastAPI, HTTPException, Depends, Request
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import JSONResponse
from typing import Dict, Any, Optional
import logging
from contextlib import asynccontextmanager
from exonware.xwsystem import (
    JsonSerializer,
    YamlSerializer,
    MsgPackSerializer,
    SecureHash,
    PathValidator,
    CircuitBreaker,
    PerformanceMonitor,
    MemoryMonitor,
    ThreadSafeFactory,
    AtomicFileWriter,
)
# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)
# Initialize xwsystem components
json_serializer = JsonSerializer()
yaml_serializer = YamlSerializer()
msgpack_serializer = MsgPackSerializer()
path_validator = PathValidator(base_path="/safe/api/data")
performance_monitor = PerformanceMonitor()
memory_monitor = MemoryMonitor(enable_auto_cleanup=True)
# Circuit breaker for external API calls
external_api_breaker = CircuitBreaker(
    failure_threshold=5,
    recovery_timeout=30,
    expected_exception=HTTPException
)
# Thread-safe factory for handlers
handler_factory = ThreadSafeFactory()
@asynccontextmanager
async def lifespan(app: FastAPI):
    """Application lifespan manager."""
    # Startup
    logger.info("Starting API server...")
    memory_monitor.start_monitoring()
    performance_monitor.start()
    yield
    # Shutdown
    logger.info("Shutting down API server...")
    memory_monitor.stop_monitoring()
    performance_monitor.stop()
app = FastAPI(
    title="Production API with xwsystem",
    description="Example production API demonstrating xwsystem features",
    version="1.0.0",
    lifespan=lifespan
)
# CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)
# Dependency for format selection


def get_serializer(format_type: str = "json"):
    """Get serializer based on format type."""
    serializers = {
        "json": json_serializer,
        "yaml": yaml_serializer,
        "msgpack": msgpack_serializer,
    }
    return serializers.get(format_type, json_serializer)
@app.get("/")
async def root():
    """Root endpoint."""
    return {
        "message": "Production API with xwsystem",
        "version": "1.0.0",
        "features": [
            "Multiple serialization formats",
            "Security validation",
            "Performance monitoring",
            "Circuit breakers",
            "Memory leak prevention"
        ]
    }
@app.get("/health")
async def health_check():
    """Health check endpoint with monitoring."""
    health_status = {
        "status": "healthy",
        "memory": memory_monitor.get_current_usage(),
        "performance": performance_monitor.get_stats(),
    }
    return health_status
@app.post("/data/{format_type}")
async def create_data(
    data: Dict[str, Any],
    format_type: str = "json",
    serializer = Depends(get_serializer)
):
    """
    Create data endpoint with format selection.
    Supports: json, yaml, msgpack
    """
    try:
        # Validate input data
        if not isinstance(data, dict):
            raise HTTPException(status_code=400, detail="Data must be a dictionary")
        # Serialize based on format
        with performance_monitor.measure("serialization"):
            serialized = serializer.dumps(data)
        # Secure storage with atomic write
        safe_path = path_validator.validate_path(f"data/{format_type}/output")
        with AtomicFileWriter(safe_path) as writer:
            writer.write(serialized)
        return {
            "status": "success",
            "format": format_type,
            "size": len(serialized),
            "message": "Data created successfully"
        }
    except Exception as e:
        logger.error(f"Error creating data: {e}")
        raise HTTPException(status_code=500, detail=str(e))
@app.get("/data/{format_type}/{data_id}")
async def get_data(
    data_id: str,
    format_type: str = "json",
    serializer = Depends(get_serializer)
):
    """Get data endpoint."""
    try:
        safe_path = path_validator.validate_path(f"data/{format_type}/{data_id}")
        with performance_monitor.measure("deserialization"):
            with open(safe_path, 'rb' if format_type == "msgpack" else 'r') as f:
                content = f.read()
                data = serializer.loads(content)
        return {
            "status": "success",
            "data": data
        }
    except FileNotFoundError:
        raise HTTPException(status_code=404, detail="Data not found")
    except Exception as e:
        logger.error(f"Error getting data: {e}")
        raise HTTPException(status_code=500, detail=str(e))
@app.post("/hash")
async def hash_data(data: Dict[str, Any]):
    """Hash data endpoint with secure hashing."""
    try:
        # Serialize data first
        data_str = json_serializer.dumps(data)
        # Create secure hash
        hash_value = SecureHash.sha256(data_str)
        return {
            "status": "success",
            "hash": hash_value,
            "algorithm": "SHA-256"
        }
    except Exception as e:
        logger.error(f"Error hashing data: {e}")
        raise HTTPException(status_code=500, detail=str(e))
@external_api_breaker
async def call_external_api(url: str):
    """External API call with circuit breaker."""
    import httpx
    async with httpx.AsyncClient() as client:
        response = await client.get(url, timeout=5.0)
        response.raise_for_status()
        return response.json()
@app.get("/external/{resource}")
async def proxy_external(resource: str):
    """Proxy external API with circuit breaker protection."""
    try:
        url = f"https://api.example.com/{resource}"
        data = await call_external_api(url)
        return {
            "status": "success",
            "data": data
        }
    except Exception as e:
        logger.error(f"External API error: {e}")
        raise HTTPException(
            status_code=503,
            detail="External service unavailable (circuit breaker)"
        )
@app.get("/metrics")
async def get_metrics():
    """Get performance metrics."""
    return {
        "performance": performance_monitor.get_stats(),
        "memory": memory_monitor.get_current_usage(),
        "circuit_breaker": {
            "state": external_api_breaker.state,
            "failures": external_api_breaker.failure_count,
        }
    }
if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
