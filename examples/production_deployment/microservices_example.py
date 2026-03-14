#!/usr/bin/env python3
#exonware/xwsystem/examples/production_deployment/microservices_example.py
"""
Microservices Example with xwsystem Circuit Breakers
This example demonstrates a microservices architecture using xwsystem
for resilience patterns, serialization, and inter-service communication.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
"""

import asyncio
import logging
from typing import Any
from fastapi import FastAPI, HTTPException
import httpx
from exonware.xwsystem import (
    CircuitBreaker,
    PerformanceMonitor,
    MemoryMonitor,
    JsonSerializer,
    MsgPackSerializer,
    ThreadSafeFactory,
    RetryConfig,
    HttpClient,
)
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)
# Initialize components
json_serializer = JsonSerializer()
msgpack_serializer = MsgPackSerializer()
performance_monitor = PerformanceMonitor()
memory_monitor = MemoryMonitor(enable_auto_cleanup=True)
http_client = HttpClient()
# Circuit breakers for each service
user_service_breaker = CircuitBreaker(
    failure_threshold=5,
    recovery_timeout=30,
    name="user_service"
)
order_service_breaker = CircuitBreaker(
    failure_threshold=5,
    recovery_timeout=30,
    name="order_service"
)
payment_service_breaker = CircuitBreaker(
    failure_threshold=3,
    recovery_timeout=60,
    name="payment_service"
)


class MicroserviceClient:
    """Base client for microservice communication."""

    def __init__(self, service_name: str, base_url: str, circuit_breaker: CircuitBreaker):
        self.service_name = service_name
        self.base_url = base_url
        self.circuit_breaker = circuit_breaker
        self.http_client = HttpClient()
        self.performance_monitor = PerformanceMonitor()
    @circuit_breaker

    async def call_service(self, endpoint: str, method: str = "GET", data: dict | None = None):
        """Call microservice with circuit breaker protection."""
        url = f"{self.base_url}/{endpoint}"
        with self.performance_monitor.measure(f"{self.service_name}_{endpoint}"):
            try:
                if method == "GET":
                    response = await self.http_client.get(url)
                elif method == "POST":
                    response = await self.http_client.post(url, json=data)
                else:
                    raise ValueError(f"Unsupported method: {method}")
                return response.json() if hasattr(response, 'json') else response
            except Exception as e:
                logger.error(f"Error calling {self.service_name}: {e}")
                raise
# Service clients
user_service = MicroserviceClient(
    "user_service",
    "http://user-service:8001",
    user_service_breaker
)
order_service = MicroserviceClient(
    "order_service",
    "http://order-service:8002",
    order_service_breaker
)
payment_service = MicroserviceClient(
    "payment_service",
    "http://payment-service:8003",
    payment_service_breaker
)
# Main API
app = FastAPI(title="Microservices Gateway with xwsystem")
@app.get("/health")
async def health_check():
    """Health check with circuit breaker status."""
    return {
        "status": "healthy",
        "services": {
            "user_service": user_service_breaker.state,
            "order_service": order_service_breaker.state,
            "payment_service": payment_service_breaker.state,
        },
        "memory": memory_monitor.get_current_usage(),
    }
@app.get("/users/{user_id}")
async def get_user(user_id: str):
    """Get user with circuit breaker protection."""
    try:
        user_data = await user_service.call_service(f"users/{user_id}")
        return {
            "status": "success",
            "data": user_data
        }
    except Exception as e:
        logger.error(f"Error getting user: {e}")
        raise HTTPException(status_code=503, detail="User service unavailable")
@app.post("/orders")
async def create_order(order_data: dict[str, Any]):
    """Create order with multi-service coordination."""
    try:
        # Validate user exists
        user_id = order_data.get("user_id")
        user = await user_service.call_service(f"users/{user_id}")
        if not user:
            raise HTTPException(status_code=404, detail="User not found")
        # Create order
        order = await order_service.call_service("orders", "POST", order_data)
        # Process payment
        payment_data = {
            "order_id": order.get("id"),
            "amount": order.get("total"),
            "user_id": user_id
        }
        payment = await payment_service.call_service("payments", "POST", payment_data)
        return {
            "status": "success",
            "order": order,
            "payment": payment
        }
    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Error creating order: {e}")
        raise HTTPException(status_code=500, detail=str(e))
@app.get("/metrics")
async def get_metrics():
    """Get aggregated metrics from all services."""
    return {
        "performance": performance_monitor.get_stats(),
        "memory": memory_monitor.get_current_usage(),
        "circuit_breakers": {
            "user_service": {
                "state": user_service_breaker.state,
                "failures": user_service_breaker.failure_count,
            },
            "order_service": {
                "state": order_service_breaker.state,
                "failures": order_service_breaker.failure_count,
            },
            "payment_service": {
                "state": payment_service_breaker.state,
                "failures": payment_service_breaker.failure_count,
            },
        }
    }
if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
