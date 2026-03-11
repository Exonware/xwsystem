#!/usr/bin/env python3
#exonware/xwsystem/examples/production_deployment/data_pipeline_example.py
"""
Data Processing Pipeline Example with xwsystem
This example demonstrates a production data processing pipeline using
xwsystem for format conversion, performance monitoring, and error handling.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
"""

import asyncio
import logging
from pathlib import Path
from typing import List, Dict, Any, Optional
from dataclasses import dataclass
from exonware.xwsystem import (
    JsonSerializer,
    YamlSerializer,
    MsgPackSerializer,
    AvroSerializer,
    ParquetSerializer,
    PerformanceMonitor,
    MemoryMonitor,
    PathValidator,
    AtomicFileWriter,
    CircularReferenceDetector,
)
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)
@dataclass


class PipelineStage:
    """Pipeline stage configuration."""
    name: str
    input_format: str
    output_format: str
    enabled: bool = True


class DataPipeline:
    """Production data processing pipeline."""

    def __init__(self, pipeline_id: str, base_path: str = "/data/pipeline"):
        self.pipeline_id = pipeline_id
        self.path_validator = PathValidator(base_path=base_path)
        self.performance_monitor = PerformanceMonitor()
        self.memory_monitor = MemoryMonitor(enable_auto_cleanup=True)
        self.circular_detector = CircularReferenceDetector()
        # Serializers
        self.serializers = {
            "json": JsonSerializer(),
            "yaml": YamlSerializer(),
            "msgpack": MsgPackSerializer(),
            "avro": AvroSerializer(),
            "parquet": ParquetSerializer(),
        }
        self.stages: List[PipelineStage] = []
        self.stats = {
            "processed": 0,
            "failed": 0,
            "total_size": 0,
        }

    def add_stage(self, stage: PipelineStage):
        """Add a processing stage."""
        self.stages.append(stage)
        logger.info(f"Added stage: {stage.name}")

    def get_serializer(self, format_type: str):
        """Get serializer for format."""
        serializer = self.serializers.get(format_type)
        if not serializer:
            raise ValueError(f"Unsupported format: {format_type}")
        return serializer

    def validate_data(self, data: Any) -> bool:
        """Validate data before processing."""
        # Check for circular references
        if self.circular_detector.is_circular(data):
            logger.warning("Circular reference detected, resolving...")
            data = self.circular_detector.resolve_circular_refs(data)
        # Check memory usage
        memory_usage = self.memory_monitor.get_current_usage()
        if memory_usage.get("percent", 0) > 90:
            logger.warning(f"High memory usage: {memory_usage}")
            return False
        return True

    async def process_stage(self, stage: PipelineStage, input_data: Any) -> Any:
        """Process a single pipeline stage."""
        if not stage.enabled:
            logger.info(f"Stage {stage.name} is disabled, skipping")
            return input_data
        logger.info(f"Processing stage: {stage.name} ({stage.input_format} -> {stage.output_format})")
        try:
            with self.performance_monitor.measure(f"stage_{stage.name}"):
                # Deserialize input
                input_serializer = self.get_serializer(stage.input_format)
                if isinstance(input_data, (str, bytes)):
                    data = input_serializer.loads(input_data)
                else:
                    data = input_data
                # Validate
                if not self.validate_data(data):
                    raise ValueError("Data validation failed")
                # Serialize output
                output_serializer = self.get_serializer(stage.output_format)
                output_data = output_serializer.dumps(data)
                # Save output
                output_path = self.path_validator.validate_path(
                    f"{self.pipeline_id}/{stage.name}/output.{stage.output_format}"
                )
                with AtomicFileWriter(output_path) as writer:
                    if isinstance(output_data, bytes):
                        writer.write(output_data)
                    else:
                        writer.write(output_data.encode('utf-8'))
                self.stats["processed"] += 1
                self.stats["total_size"] += len(output_data) if isinstance(output_data, bytes) else len(output_data.encode())
                logger.info(f"Stage {stage.name} completed successfully")
                return output_data
        except Exception as e:
            logger.error(f"Error in stage {stage.name}: {e}")
            self.stats["failed"] += 1
            raise

    async def run(self, initial_data: Any, initial_format: str = "json"):
        """Run the complete pipeline."""
        logger.info(f"Starting pipeline: {self.pipeline_id}")
        self.memory_monitor.start_monitoring()
        self.performance_monitor.start()
        try:
            current_data = initial_data
            current_format = initial_format
            for stage in self.stages:
                if stage.input_format != current_format:
                    # Convert format if needed
                    converter = self.get_serializer(current_format)
                    data_obj = converter.loads(current_data) if isinstance(current_data, (str, bytes)) else current_data
                    serializer = self.get_serializer(stage.input_format)
                    current_data = serializer.dumps(data_obj)
                    current_format = stage.input_format
                current_data = await self.process_stage(stage, current_data)
                current_format = stage.output_format
            logger.info(f"Pipeline {self.pipeline_id} completed successfully")
            return current_data
        finally:
            self.memory_monitor.stop_monitoring()
            self.performance_monitor.stop()

    def get_stats(self) -> Dict[str, Any]:
        """Get pipeline statistics."""
        return {
            "pipeline_id": self.pipeline_id,
            "stats": self.stats,
            "performance": self.performance_monitor.get_stats(),
            "memory": self.memory_monitor.get_current_usage(),
        }
# Example usage
async def main():
    """Example pipeline execution."""
    # Create pipeline
    pipeline = DataPipeline("example_pipeline", base_path="/tmp/pipeline_data")
    # Define stages
    pipeline.add_stage(PipelineStage(
        name="ingest",
        input_format="json",
        output_format="msgpack"
    ))
    pipeline.add_stage(PipelineStage(
        name="transform",
        input_format="msgpack",
        output_format="avro"
    ))
    pipeline.add_stage(PipelineStage(
        name="analytics",
        input_format="avro",
        output_format="parquet"
    ))
    # Sample data
    sample_data = {
        "users": [
            {"id": 1, "name": "Alice", "age": 30},
            {"id": 2, "name": "Bob", "age": 25},
        ],
        "metadata": {
            "timestamp": "2025-01-01T00:00:00Z",
            "version": "1.0"
        }
    }
    json_serializer = JsonSerializer()
    initial_data = json_serializer.dumps(sample_data)
    # Run pipeline
    result = await pipeline.run(initial_data, initial_format="json")
    # Print stats
    stats = pipeline.get_stats()
    print(f"Pipeline Stats: {stats}")
if __name__ == "__main__":
    asyncio.run(main())
