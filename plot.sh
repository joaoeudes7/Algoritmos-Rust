#!/bin/bash

cd ./results_time

# Plot svgs (better to latex and quality)
rm -rf graphs
mkdir -p graphs

gnuplot -persist <<-EOFMarker
    set grid;
    set pointsize 1

    set term svg

    set ylabel "Secs (less is better)"
    set xlabel "Items"

    folderout = 'graphs'

    set output "graphs/quick.svg"
    plot 'quick_b.txt' using 1:2 smooth bezier with lines, 'quick_m.txt' using 1:2 smooth bezier with lines, 'quick_w.txt' using 1:2 smooth bezier with lines

    set output "graphs/quick_b.svg"
    plot 'quick_b.txt' using 1:2 smooth bezier with lines

    set output "graphs/quick_m.svg"
    plot 'quick_m.txt' using 1:2 smooth bezier with lines

    set output "graphs/quick_w.svg"
    plot 'quick_w.txt' using 1:2 smooth bezier with lines



    set output "graphs/merge.svg"
    plot 'merge_b.txt' using 1:2 smooth bezier with lines, 'merge_m.txt' using 1:2 smooth bezier with lines, 'merge_w.txt' using 1:2 smooth bezier with lines

    set output "graphs/merge_b.svg"
    plot 'merge_b.txt' using 1:2 smooth bezier with lines

    set output "graphs/merge_m.svg"
    plot 'merge_m.txt' using 1:2 smooth bezier with lines

    set output "graphs/merge_w.svg"
    plot 'merge_w.txt' using 1:2 smooth bezier with lines



    set output "graphs/insertion.svg"
    plot 'insertion_b.txt' using 1:2 smooth bezier with lines, 'insertion_m.txt' using 1:2 smooth bezier with lines, 'insertion_w.txt' using 1:2 smooth bezier with lines

    set output "graphs/insertion_b.svg"
    plot 'insertion_b.txt' using 1:2 smooth bezier with lines

    set output "graphs/insertion_m.svg"
    plot 'insertion_m.txt' using 1:2 smooth bezier with lines

    set output "graphs/insertion_w.svg"
    plot 'insertion_w.txt' using 1:2 smooth bezier with lines



    set output "graphs/comparation_b.svg"
    plot 'quick_b.txt' using 1:2 smooth bezier with lines, 'insertion_b.txt' using 1:2 smooth bezier with lines, 'merge_b.txt' using 1:2 smooth bezier with lines

    set output "graphs/comparation_m.svg"
    plot 'quick_m.txt' using 1:2 smooth bezier with lines, 'insertion_m.txt' using 1:2 smooth bezier with lines, 'merge_m.txt' using 1:2 smooth bezier with lines

    set output "graphs/comparation_w.svg"
    plot 'quick_w.txt' using 1:2 smooth bezier with lines, 'insertion_w.txt' using 1:2 smooth bezier with lines, 'merge_w.txt' using 1:2 smooth bezier with lines
EOFMarker
