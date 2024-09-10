# Define the data file
set datafile separator tab

# Basic chart settings
set title "Speedychain Size Growth" font ",18"
set xlabel "Number of Transactions" font ",16"
set ylabel "Size (MB)" font ",16"
set key left top
set key font ",16"
set grid

# Settings for the x-axis (logarithmic for better visualization)
set logscale x 2
set format x "2^{%L}"
set xtics (0, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216)

# Settings for the y-axis (logarithmic for better visualization)

set logscale y 2
set format y "2^{%L}"
set ytics (0, 1,2,4,8,16,32,64,128,256,512, 1024, 2048, 4096, 8192, 16384, 32768, 65536)

# Define line styles
set style line 1 lc rgb '#dd181f' lt 1 lw 1.5 pt 7 ps 1.5  # Red line, solid, width 1.5, point 7 (circle), point size 1.5
set style line 2 lc rgb '#0060ad' lt 1 lw 1.5 pt 5 ps 1.5  # Blue line, solid, width 1.5, point 5 (square), point size 1.5

# Load data from file and plot
plot "data.txt" using 1:3 with linespoints linestyle 1 title "Speedychain w/o Stordy", \
     "data.txt" using 1:($2+$4) with linespoints linestyle 2 title "Speedychain with Stordy + Stordy"
