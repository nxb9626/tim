#!/bin/bash
cd $(dirname "$0")
cd ./sql

# Don't always want all
if [[ $1 == "all" ]]; then
  echo
  echo "Total Times of Activities:";
  cat total.sql | sqlite3 > ../cache/total.csv;
  echo
  column -s, -t < ../cache/total.csv | tr "\"" " " | tr "\+" " ";
fi;

echo
echo "Today's Activity Times:";
echo

cat today.sql | sqlite3 > ../cache/today.csv;

column -s, -t < ../cache/today.csv | tr "\"" " " | tr "\+" " ";

cat running.sql | sqlite3 > ../cache/running.csv;

if [[ $(cat ../cache/running.csv | wc -l) != "0" ]]; then
  echo
  echo "Activities Running:"
  echo
  column -s, -t < ../cache/running.csv | tr "\"" " " | tr "\+" " ";
  echo
else
  echo
  echo "No Activities Running."
  echo
fi;
