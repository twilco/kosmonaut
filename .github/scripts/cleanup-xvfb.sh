#!/bin/bash
# Author of this script Bret Comnes.
xvfb_pids=`ps aux | grep tmp/xvfb-run | grep -v grep | awk '{print $2}'`
if [ "$xvfb_pids" != "" ]; then
    echo "Cleaning up the following xvfb processes: $xvfb_pids"
    sudo kill "$xvfb_pids"
else
    echo "No xvfb processes to kill"
fi
