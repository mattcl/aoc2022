#!/bin/bash

rm -rf bench-out
mkdir bench-out

echo "day,time" > combined.csv

i=1
while [ $i -ne 26 ]
do
  padded=`printf %03d $i`
  short_pad=`printf %02d $i`
  cp target/criterion/$padded*/*parsing*/base/estimates.json bench-out/day_$padded.json
  t=$(cat bench-out/day_$padded.json | jq .mean.point_estimate)

  echo "${short_pad},${t}" >> combined.csv

  # xsv select sample_measured_value bench-out/day_$padded.csv | xsv stats > bench-out/stats_day_$i.csv
  # sed -i "s/field/day/g" bench-out/stats_day_$i.csv
  # sed -i "s/sample_measured_value/${short_pad}/g" bench-out/stats_day_$i.csv
  i=`expr $i + 1`
done

xsv sort combined.csv > combined_sorted.csv
xsv select time combined_sorted.csv > combined_plain.csv
