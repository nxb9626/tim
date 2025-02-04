#!/bin/bash
#
# Input for time tracking csv stored in ./db/
# w
# 'start' and 'stop's can be re-ran on same activity
# so duplicates must be merged on read of csv
#
# 'all' should only be passed in for ending 'all' activities


set -e
cd $(dirname "$0")

if [[ "$1" != "start" && "$1" != "stop"  ]]; then
  echo "Must provide 'start' or 'stop'." && exit 1;
fi

if [[ "$2" == "" ]]; then
  echo "Must provide activity" && exit 1;
fi

if [[ "$2" == "all" && $1 == "start" ]]; then
  echo "Use 'all' to stop all activities" && exit 1;
fi

# date --iso-8601=seconds isn't cross platform really
echo "$(date +"%Y-%m-%dT%H:%M:%S%:z"),$1,$2" >> ./db/db.csv

if [[ "$1" == "start" ]]; then
  echo "'$2' has been started" && exit 0;
fi

if [[ "$1" == "stop" ]]; then
  echo "'$2' has been stopped" && exit 0;
fi
