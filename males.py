import os
import random

def create_corrupt_file(name, fmt, size):
    with open(f"{name}.{fmt}", "wb") as f:
        f.write(os.urandom(size))
    print(f"File '{name}.{fmt}' of size {size} bytes has been created.")

if __name__ == "__main__":
    name = input("Name: ")
    fmt = input("Format (e.g., doc, jpg, mp4): ")
    size = int(input("Size (in bytes, e.g., 1024): "))
    create_corrupt_file(name, fmt, size)
