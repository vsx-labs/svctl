set -euox

echo "hello {{ username }}!"

echo "Starting loop..."
for i in {1..3}; do
    echo "Processing item $i"
    sleep 1
done
echo "Loop completed!"
