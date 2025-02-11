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

if [[ "$1" == "" ]]; then
  echo "Must provide activity" && exit 1;
fi

# date --iso-8601=seconds isn't cross platform really
echo "$(date +"%Y-%m-%dT%H:%M:%S%:z"),stop,all" >> ./db/db.csv
echo "$(date +"%Y-%m-%dT%H:%M:%S%:z"),start,$1" >> ./db/db.csv

echo "pivoted to '$1'" && exit 0;

# if [[ "$1" == "stop" ]]; then
#   echo "'$2' has been stopped" && exit 0;
# fi
