#!/bin/bash

limit=$(cat /sys/fs/cgroup/memory/memory.limit_in_bytes)

echo "scale=2; $limit/(1024*1024)" | bc
