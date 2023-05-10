set terminal svg size 1280,800 noenhanced background rgb 'white'
set output 'benchmark/benchmark.svg'

set style data histogram
set style histogram cluster gap 1

set style fill solid border rgb "black"
set auto x
set yrange [0:*]
set ylabel "Milliseconds"
set xlabel "Test cases"
plot 'benchmark/benchmark.dat' using 2:xtic(1) title col, \
        '' using 3:xtic(1) title col
