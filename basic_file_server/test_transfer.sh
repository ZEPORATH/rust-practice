#!/bin/bash

# Test script for file server

cd /Users/sshekhar/workspace/rust-practice/basic_file_server

echo "Starting server..."
cargo run --bin basic_file_server server '127.0.0.1:4000' . > /tmp/server.log 2>&1 &
SERVER_PID=$!

sleep 2

echo "Testing client with bigfile.bin (100MB)..."
rm -f /tmp/bigfile.bin
cargo run --bin basic_file_server client --addr '127.0.0.1:4000' --get bigfile.bin --out /tmp 2>&1

echo ""
echo "Killing server..."
kill $SERVER_PID 2>/dev/null
wait $SERVER_PID 2>/dev/null

echo ""
echo "Checking downloaded file..."
if [ -f /tmp/bigfile.bin ]; then
    echo "✓ File downloaded successfully!"
    ls -lh bigfile.bin /tmp/bigfile.bin
    
    echo ""
    echo "Comparing MD5 checksums..."
    ORIG_MD5=$(md5 -q bigfile.bin)
    DOWN_MD5=$(md5 -q /tmp/bigfile.bin)
    echo "Original:   $ORIG_MD5"
    echo "Downloaded: $DOWN_MD5"
    
    if [ "$ORIG_MD5" = "$DOWN_MD5" ]; then
        echo "✓ MD5 checksums match!"
    else
        echo "✗ MD5 checksums DO NOT match!"
    fi
else
    echo "✗ File not downloaded"
    echo ""
    echo "Server log:"
    tail -20 /tmp/server.log
fi

