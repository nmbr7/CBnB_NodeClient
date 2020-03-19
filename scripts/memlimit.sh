#!/bin/bash

limit=$(cat /sys/fs/cgroup/memory/memory.limit_in_bytes)

echo $limit
