echo "Name :"
read NAME
echo ""
echo "Format :"
read FORMAT
echo ""
echo "Size :"
read SIZE
echo ""
dd if=/dev/null of="$NAME.$FORMAT" count=0 seek=$SIZE
