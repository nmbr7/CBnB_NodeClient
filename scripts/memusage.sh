#!/bin/bash

use=$(cat /sys/fs/cgroup/memory/memory.usage_in_bytes)
limit=$(cat /sys/fs/cgroup/memory/memory.limit_in_bytes)

echo "scale=10; $use/($limit/1024)*100" | bc

