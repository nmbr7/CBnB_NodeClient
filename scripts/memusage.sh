#!/bin/bash

use=$(cat /sys/fs/cgroup/memory/memory.usage_in_bytes)
limit=$(cat /sys/fs/cgroup/memory/memory.limit_in_bytes)

echo "scale=4; $use/$limit*100" | bc

