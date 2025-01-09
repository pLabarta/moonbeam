#!/bin/bash
FILES=$1
OUTPUT=$2
for file in ${FILES}; do
  subweight compare commits --threshold 1 --strip-path-prefix ".*/" --format csv --no-color --method exact-worst --path-pattern $file base_ref head_ref | sed 1d >> $OUTPUT
done