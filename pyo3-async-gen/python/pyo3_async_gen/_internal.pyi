from __future__ import annotations

class Counter:
    def __init__(self, count_to: int = 5) -> None:
        """init method"""

    def __iter__(self) -> Counter:
        """iter magic method"""

    def __next__(self) -> int:
        """next magic method"""

class AsyncCounter:
    def __init__(self, count_to: int = 5) -> None:
        """init method"""

    def __await__(self) -> AsyncCounter:
        """async magic method"""

    def __aiter__(self) -> AsyncCounter:
        """aiter magic method"""

    async def __anext__(self) -> int:
        """anext magic method"""
