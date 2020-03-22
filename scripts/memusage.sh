#!/bin/bash

use=$(cat /sys/fs/cgroup/memory/memory.usage_in_bytes)
cache=$(cat /sys/fs/cgroup/memory/memory.stat|head -n20|tail -n1|awk '{print $2}')
limit=$(cat /sys/fs/cgroup/memory/memory.limit_in_bytes)

echo "scale=10; ($use-$cache)/($limit)*100" | bc


