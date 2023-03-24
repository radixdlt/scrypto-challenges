#!/bin/bash
exec >log.txt 2>&1

# Runs the relay for each registered member
file="dao_members.txt"
# read line by line
while IFS= read -r line
do
  echo "Running relay for $line"
  # put a sleep time here to avoid rate limiting
  sleep 1
  python inatty.py $line
done < "$file"