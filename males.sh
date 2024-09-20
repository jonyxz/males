#!/bin/bash

echo "Name:"
read NAME
echo ""
echo "Format (e.g., doc, jpg, mp4):"
read FORMAT
echo ""
echo "Size (in bytes, e.g., 1024):"
read SIZE
echo ""

dd if=/dev/urandom of="$NAME.$FORMAT" bs=1 count=$SIZE
echo "File '$NAME.$FORMAT' of size $SIZE bytes has been created."
