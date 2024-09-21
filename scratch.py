from __future__ import annotations

import asyncio
from pyo3_async_gen import Counter
from pyo3_async_gen import AsyncCounter as RustAsyncCounter


class NativeAsyncCounter:
    curr: int
    count_to: int

    def __init__(self, count_to: int = 5) -> None:
        self.curr = 0
        self.count_to = count_to

    def __aiter__(self) -> NativeAsyncCounter:
        return self

    async def __anext__(self) -> int:
        if self.curr >= self.count_to:
            raise StopAsyncIteration("exhausted")

        self.curr += 1
        return self.curr


async def async_main():
    async for val in RustAsyncCounter():
        print(f"async val: {val}")


async def native_async_main():
    async for val in NativeAsyncCounter():
        print(f"async val: {val}")


def main():
    for val in Counter():
        print(f"val: {val}")


if __name__ == "__main__":
    # print("starting sync main...\n")
    # main()

    # print("starting native async main...\n")
    # asyncio.run(native_async_main())

    print("starting rust async main...\n")
    asyncio.run(async_main())
