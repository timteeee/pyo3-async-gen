from pyo3_async_gen import Counter


def main():
    for val in Counter():
        print(f"val: {val}")


if __name__ == "__main__":
    main()
