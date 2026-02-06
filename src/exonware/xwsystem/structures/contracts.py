#exonware/xwsystem/src/exonware/xwsystem/structures/contracts.py
"""
Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.1.0.4
Generation Date: September 04, 2025

Structures module contracts - interfaces and enums for data structure functionality.
"""

from __future__ import annotations

from typing import Any, Optional, Iterator, Callable, Protocol, runtime_checkable

# Import enums from types module
from .defs import (
    StructureType,
    TraversalOrder,
    TraversalType,
    GraphType,
    CircularDetectionMethod,
    ValidationLevel
)


@runtime_checkable
class ITreeNode(Protocol):
    """Interface for tree node operations."""
    
    @property
    def value(self) -> Any:
        """Node value."""
        ...
    
    @property
    def children(self) -> list[ITreeNode]:
        """Node children."""
        ...
    
    @property
    def parent(self) -> Optional[ITreeNode]:
        """Node parent."""
        ...
    
    def add_child(self, child: ITreeNode) -> None:
        """Add child node."""
        ...
    
    def remove_child(self, child: ITreeNode) -> None:
        """Remove child node."""
        ...
    
    def is_leaf(self) -> bool:
        """Check if node is leaf."""
        ...
    
    def get_depth(self) -> int:
        """Get node depth."""
        ...


@runtime_checkable
class ITreeWalker(Protocol):
    """Interface for tree walking operations."""
    
    def walk_preorder(self, root: ITreeNode) -> Iterator[ITreeNode]:
        """Walk tree in preorder."""
        ...
    
    def walk_inorder(self, root: ITreeNode) -> Iterator[ITreeNode]:
        """Walk tree in inorder."""
        ...
    
    def walk_postorder(self, root: ITreeNode) -> Iterator[ITreeNode]:
        """Walk tree in postorder."""
        ...
    
    def walk_level_order(self, root: ITreeNode) -> Iterator[ITreeNode]:
        """Walk tree in level order."""
        ...
    
    def find_nodes(self, root: ITreeNode, predicate: Callable[[ITreeNode], bool]) -> list[ITreeNode]:
        """Find nodes matching predicate."""
        ...


@runtime_checkable
class ICircularDetector(Protocol):
    """Interface for circular reference detection."""
    
    def detect_circular_reference(self, obj: Any) -> bool:
        """Detect circular reference in object."""
        ...
    
    def find_circular_paths(self, obj: Any) -> list[list[Any]]:
        """Find all circular paths."""
        ...
    
    def get_circular_objects(self, obj: Any) -> list[Any]:
        """Get objects involved in circular references."""
        ...
    
    def break_circular_references(self, obj: Any) -> Any:
        """Break circular references."""
        ...


@runtime_checkable
class IGraphNode(Protocol):
    """Interface for graph node operations."""
    
    @property
    def id(self) -> str:
        """Node ID."""
        ...
    
    @property
    def data(self) -> Any:
        """Node data."""
        ...
    
    @property
    def neighbors(self) -> list[IGraphNode]:
        """Node neighbors."""
        ...
    
    def add_neighbor(self, neighbor: IGraphNode, weight: Optional[float] = None) -> None:
        """Add neighbor node."""
        ...
    
    def remove_neighbor(self, neighbor: IGraphNode) -> None:
        """Remove neighbor node."""
        ...
    
    def get_edge_weight(self, neighbor: IGraphNode) -> Optional[float]:
        """Get edge weight to neighbor."""
        ...


@runtime_checkable
class IGraph(Protocol):
    """Interface for graph operations."""
    
    def add_node(self, node: IGraphNode) -> None:
        """Add node to graph."""
        ...
    
    def remove_node(self, node_id: str) -> None:
        """Remove node from graph."""
        ...
    
    def get_node(self, node_id: str) -> Optional[IGraphNode]:
        """Get node by ID."""
        ...
    
    def add_edge(self, from_node: str, to_node: str, weight: Optional[float] = None) -> None:
        """Add edge between nodes."""
        ...
    
    def remove_edge(self, from_node: str, to_node: str) -> None:
        """Remove edge between nodes."""
        ...
    
    def get_all_nodes(self) -> list[IGraphNode]:
        """Get all nodes."""
        ...
    
    def get_all_edges(self) -> list[tuple]:
        """Get all edges."""
        ...
    
    def is_connected(self, from_node: str, to_node: str) -> bool:
        """Check if nodes are connected."""
        ...
