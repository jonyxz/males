#!/bin/bash

echo "Name:"
read NAME
echo ""
echo "Format (contoh: txt, jpg, mp4, dll):"
read FORMAT
echo ""
echo "Size (dalam bytes, contoh: 1024):"
read SIZE
echo ""

dd if=/dev/urandom of="$NAME.$FORMAT" bs=1 count=$SIZE
echo "File '$NAME.$FORMAT' sebesar $SIZE bytes telah dibuat."
