#!/bin/bash


tstart=$(date +%s%N)
cstart=$(cat /sys/fs/cgroup/cpuacct/cpuacct.usage)

sleep 1

tstop=$(date +%s%N)
cstop=$(cat /sys/fs/cgroup/cpuacct/cpuacct.usage)

bc -l <<EOF
(($cstop - $cstart) / ($tstop - $tstart) * 100)/4
EOF
