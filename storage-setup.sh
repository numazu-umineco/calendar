#!/bin/sh

handle_sigint() {
    echo "Ctrl+C detected. Exiting..."
    exit 1
}

trap 'handle_sigint' INT

while true; do
    mc config host add storage http://storage:9000 "${MINIO_ACCSES_KEY_ID}" "${MINIO_SECRET_ACCESS_KEY}" > /dev/null 2>&1
    if [ $? -eq 0 ]; then
        break
    fi
    echo "Waiting for storage to be ready..."
    sleep 1
done
mc mb --with-lock storage/database > /dev/null 2>&1 || echo "Bucket "database" is already exists, ignoring error"
mc mb --with-lock storage/umineco-calendar > /dev/null 2>&1 || echo "Bucket "umineco-calendar" is already exists, ignoring error"
