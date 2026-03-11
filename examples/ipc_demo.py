#!/usr/bin/env python3
#exonware/xwsystem/examples/ipc_demo.py
"""
🚀 xSystem IPC (Inter-Process Communication) Demo
Showcasing all IPC features for production-grade multiprocessing.
"""

import asyncio
import time
import multiprocessing as mp
from datetime import datetime
print("🚀 xSystem IPC Demo")
print("=" * 50)
# 1. ✅ PROCESS MANAGEMENT
print("\n1. ✅ PROCESS MANAGEMENT")
print("-" * 30)
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))
from exonware.xwsystem import ProcessManager

def demo_process_management():
    """Demonstrate process management capabilities."""
    with ProcessManager(max_processes=3) as manager:
        # Start some demo processes
        print("Starting demo processes...")
        # Start a simple echo process
        success1 = manager.start_process("echo_demo", ["echo", "Hello from process!"])
        print(f"✓ Echo process started: {success1}")
        # Start a sleep process (cross-platform)
        if mp.current_process().name == 'MainProcess':
            import sys
            if sys.platform == 'win32':
                success2 = manager.start_process("sleep_demo", ["timeout", "2"])
            else:
                success2 = manager.start_process("sleep_demo", ["sleep", "2"])
            print(f"✓ Sleep process started: {success2}")
        # List all processes
        processes = manager.list_processes()
        print(f"✓ Active processes: {len(processes)}")
        for proc in processes:
            print(f"  - {proc.name} (PID: {proc.pid}, Status: {proc.status})")
        # Wait a bit and check status
        time.sleep(1)
        # Stop processes gracefully
        for proc in processes:
            if manager.is_running(proc.name):
                manager.stop_process(proc.name, timeout=5.0)
                print(f"✓ Stopped process: {proc.name}")
demo_process_management()
# 2. ✅ SHARED MEMORY
print("\n2. ✅ SHARED MEMORY")
print("-" * 30)
from exonware.xwsystem import SharedMemoryManager, SharedData

def demo_shared_memory():
    """Demonstrate shared memory functionality."""
    # Single shared data segment
    with SharedData("demo_segment", size=1024) as segment:
        # Store complex data
        demo_data = {
            "timestamp": time.time(),
            "message": "Hello from shared memory!",
            "numbers": [1, 2, 3, 4, 5],
            "nested": {"key": "value", "count": 42}
        }
        success = segment.set(demo_data)
        print(f"✓ Data stored in shared memory: {success}")
        # Retrieve data
        retrieved = segment.get()
        print(f"✓ Data retrieved: {retrieved['message']}")
        print(f"✓ Numbers sum: {sum(retrieved['numbers'])}")
        # Clear segment
        segment.clear()
        empty_data = segment.get()
        print(f"✓ Segment cleared: {empty_data is None}")
    # Multiple segments with manager
    with SharedMemoryManager() as manager:
        # Create multiple segments
        seg1 = manager.create_segment("config", 512)
        seg2 = manager.create_segment("cache", 1024)
        seg3 = manager.create_segment("logs", 2048)
        # Store different types of data
        from exonware.xwsystem.version import __version__
        seg1.set({"app": "xSystem", "version": __version__})
        seg2.set({"cached_results": ["result1", "result2", "result3"]})
        seg3.set({"log_entries": [f"Entry {i}" for i in range(10)]})
        segments = manager.list_segments()
        print(f"✓ Created {len(segments)} shared memory segments")
        # Verify data
        config = seg1.get()
        cache = seg2.get()
        logs = seg3.get()
        print(f"✓ Config: {config['app']} v{config['version']}")
        print(f"✓ Cache: {len(cache['cached_results'])} items")
        print(f"✓ Logs: {len(logs['log_entries'])} entries")
demo_shared_memory()
# 3. ✅ MESSAGE QUEUES
print("\n3. ✅ MESSAGE QUEUES")
print("-" * 30)
from exonware.xwsystem import MessageQueue, AsyncMessageQueue

def demo_message_queues():
    """Demonstrate message queue functionality."""
    # Thread-safe queue
    with MessageQueue(maxsize=100, enable_priority=True) as queue:
        # Send messages with different priorities
        messages = [
            ("Critical alert", 1),
            ("Normal message", 5),
            ("Low priority task", 10),
            ("Another critical", 1),
            ("Info message", 7)
        ]
        for msg, priority in messages:
            success = queue.put(msg, priority=priority)
            print(f"✓ Queued: '{msg}' (priority: {priority})")
        print(f"✓ Queue size: {queue.size()}")
        # Receive messages (should come out by priority)
        received = []
        while not queue.empty():
            msg = queue.get_nowait()
            if msg:
                received.append(msg)
        print(f"✓ Received {len(received)} messages:")
        for i, msg in enumerate(received, 1):
            print(f"  {i}. {msg}")
        # Show statistics
        stats = queue.get_stats()
        print(f"✓ Queue stats: {stats['messages_sent']} sent, {stats['messages_received']} received")
demo_message_queues()
async def demo_async_message_queues():
    """Demonstrate async message queue functionality."""
    async with AsyncMessageQueue(maxsize=50) as queue:
        # Send async messages
        messages = ["Async msg 1", "Async msg 2", "Async msg 3"]
        for msg in messages:
            success = await queue.put(msg)
            print(f"✓ Async queued: '{msg}'")
        print(f"✓ Async queue size: {queue.size()}")
        # Receive async messages
        received = []
        while not queue.empty():
            msg = queue.get_nowait()
            if msg:
                received.append(msg)
        print(f"✓ Async received {len(received)} messages")
        # Show async stats
        stats = await queue.get_stats()
        print(f"✓ Async stats: {stats['messages_sent']} sent, {stats['messages_received']} received")
asyncio.run(demo_async_message_queues())
# 4. ✅ PROCESS POOLS
print("\n4. ✅ PROCESS POOLS")
print("-" * 30)
from exonware.xwsystem import ProcessPool, AsyncProcessPool

def cpu_intensive_task(n):
    """Simulate CPU-intensive work."""
    result = 0
    for i in range(n * 100000):
        result += i % 7
    return result

def demo_process_pools():
    """Demonstrate process pool functionality."""
    with ProcessPool(max_workers=mp.cpu_count()) as pool:
        # Submit multiple CPU-intensive tasks
        tasks = []
        task_data = [10, 20, 30, 40, 50]
        print(f"Submitting {len(task_data)} CPU-intensive tasks...")
        start_time = time.time()
        for i, data in enumerate(task_data):
            task_id = pool.submit(cpu_intensive_task, data, task_id=f"cpu_task_{i}")
            tasks.append(task_id)
            print(f"✓ Submitted task {task_id}")
        # Wait for all tasks to complete
        results = pool.wait_for_all(timeout=30.0)
        end_time = time.time()
        print(f"✓ Completed {len(results)} tasks in {end_time - start_time:.2f} seconds")
        # Show successful results
        successful = [r for r in results if r.success]
        failed = [r for r in results if not r.success]
        print(f"✓ Successful: {len(successful)}, Failed: {len(failed)}")
        for result in successful[:3]:  # Show first 3 results
            print(f"  Task {result.task_id}: {result.result} (took {result.execution_time:.3f}s)")
        # Show pool statistics
        stats = pool.get_stats()
        print(f"✓ Pool stats: {stats['tasks_completed']} completed, avg time: {stats.get('avg_execution_time', 0):.3f}s")
demo_process_pools()
async def demo_async_process_pools():
    """Demonstrate async process pool functionality."""
    def fibonacci(n):
        """Calculate fibonacci number."""
        if n <= 1:
            return n
        return fibonacci(n-1) + fibonacci(n-2)
    async with AsyncProcessPool(max_workers=2) as pool:
        # Submit fibonacci calculations
        fib_numbers = [25, 26, 27, 28]
        tasks = []
        print(f"Submitting {len(fib_numbers)} Fibonacci calculations...")
        start_time = time.time()
        for num in fib_numbers:
            task_id = await pool.submit(fibonacci, num)
            tasks.append((task_id, num))
            print(f"✓ Submitted Fibonacci({num})")
        # Get results
        results = []
        for task_id, num in tasks:
            try:
                result = await pool.get_result(task_id, timeout=10.0)
                results.append((num, result))
                print(f"✓ Fibonacci({num}) = {result}")
            except Exception as e:
                print(f"✗ Fibonacci({num}) failed: {e}")
        end_time = time.time()
        print(f"✓ Async pool completed in {end_time - start_time:.2f} seconds")
asyncio.run(demo_async_process_pools())
# 5. ✅ PIPES
print("\n5. ✅ PIPES")
print("-" * 30)
from exonware.xwsystem import Pipe, AsyncPipe

def demo_pipes():
    """Demonstrate pipe functionality."""
    # Note: Pipes are primarily for inter-process communication
    # This demo shows the API, but real usage would be between processes
    with Pipe(duplex=True) as pipe:
        # Test data
        test_data = {
            "command": "process_data",
            "payload": {"numbers": [1, 2, 3, 4, 5], "operation": "sum"},
            "timestamp": time.time()
        }
        print("✓ Created bidirectional pipe")
        # In a real scenario, this would be in separate processes
        # For demo purposes, we show the API usage
        print(f"✓ Pipe ready for communication")
        print(f"✓ Would send: {test_data['command']}")
        print(f"✓ Payload size: {len(str(test_data))}")
demo_pipes()
async def demo_async_pipes():
    """Demonstrate async pipe functionality."""
    pipe = AsyncPipe(buffer_size=4096)
    try:
        # In a real scenario, you'd connect to another process
        print("✓ Created async pipe")
        print("✓ Buffer size: 4096 bytes")
        print("✓ Ready for async inter-process communication")
    finally:
        await pipe.close()
asyncio.run(demo_async_pipes())
# 6. ✅ INTEGRATION EXAMPLE
print("\n6. ✅ INTEGRATION EXAMPLE")
print("-" * 30)

def demo_ipc_integration():
    """Demonstrate integrated IPC functionality."""
    print("🔄 Producer-Consumer Pattern with IPC")
    # Create shared configuration
    with SharedData("app_config", 512) as config:
        config.set({
            "batch_size": 10,
            "max_workers": 2,
            "timeout": 5.0,
            "started_at": datetime.now().isoformat()
        })
        # Create message queue for tasks
        with MessageQueue(maxsize=20) as task_queue:
            # Create process pool for workers
            with ProcessPool(max_workers=2) as worker_pool:
                # Produce tasks
                print("📤 Producing tasks...")
                for i in range(5):
                    task = f"process_item_{i}"
                    task_queue.put(task, priority=i)
                    print(f"  ✓ Queued: {task}")
                # Process tasks
                print("⚙️  Processing tasks...")
                results = []
                while not task_queue.empty():
                    task = task_queue.get_nowait()
                    if task:
                        # Submit to process pool
                        task_id = worker_pool.submit(lambda x: f"processed_{x}", task)
                        result = worker_pool.get_result(task_id, timeout=5.0)
                        if result and result.success:
                            results.append(result.result)
                            print(f"  ✓ Completed: {result.result}")
                # Show final results
                app_config = config.get()
                print(f"📊 Final Results:")
                print(f"  ✓ Config batch size: {app_config['batch_size']}")
                print(f"  ✓ Processed items: {len(results)}")
                print(f"  ✓ Started at: {app_config['started_at']}")
demo_ipc_integration()
# FINAL SUMMARY
print("\n" + "=" * 50)
print("🎉 IPC DEMONSTRATION COMPLETE!")
print("=" * 50)
ipc_features = [
    "✅ Process Management (start, stop, monitor, graceful shutdown)",
    "✅ Shared Memory (cross-process data sharing with serialization)",
    "✅ Message Queues (thread-safe & async with priority support)",
    "✅ Process Pools (parallel processing with monitoring)",
    "✅ Pipes (bidirectional inter-process communication)",
    "✅ Async Support (full asyncio integration for all components)",
    "✅ Error Handling (timeouts, graceful degradation, recovery)",
    "✅ Statistics (performance monitoring and usage tracking)",
    "✅ Cross-Platform (Windows & Unix compatibility)",
    "✅ Production-Ready (context managers, proper cleanup)"
]
for feature in ipc_features:
    print(feature)
print("\n🚀 xSystem IPC provides enterprise-grade:")
print("   • Multi-processing coordination")
print("   • High-performance data sharing") 
print("   • Fault-tolerant communication")
print("   • Scalable parallel processing")
print("   • Production monitoring & control")
print("\n🎯 Ready for distributed computing!")
