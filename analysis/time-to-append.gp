# Define the data file
set datafile separator tab

# Basic chart settings
set title "Time to add a transaction" font ",18"
set xlabel "Number of Transactions" font ",16"
set ylabel "Time (ms)" font ",16"
set key left top
set key font ",16"
set grid

# Settings for the x-axis (logarithmic for better visualization)
set logscale x 2
set format x "2^{%L}"
set xtics (0, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216) font ",10"

# Settings for the y-axis (to highlight the stability of transaction times)
set yrange [0:8]
set ytics 1 font ",10"

# Define line styles
set style line 1 lc rgb '#0060ad' lt 1 lw 2  # Purple line, dotted, width 2

# Load data from file and plot
plot "data.txt" using 1:6 with lines linestyle 1 title "Transaction Time"
