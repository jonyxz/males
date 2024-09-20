import os
import random

def create_corrupt_file(name, fmt, size):
    with open(f"{name}.{fmt}", "wb") as f:
        f.write(os.urandom(size))
    print(f"File '{name}.{fmt}' sebesar {size} bytes telah dibuat.")

if __name__ == "__main__":
    name = input("Name: ")
    fmt = input("Format (contoh: doc, jpg, mp4): ")
    size = int(input("Size (dalam bytes, contoh: 1024): "))
    create_corrupt_file(name, fmt, size)
