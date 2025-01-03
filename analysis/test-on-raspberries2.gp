# Define the data file
set datafile separator whitespace

# Basic chart settings

set title "SpeedyChain Disk Consumption in IoT Environment" font ",18"
set xlabel "Number of Transactions" font ",16"
set ylabel "Memory Size (MB)" font ",16"

set key left top
set key font ",16"
set grid


# set xtics (0, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216)

# set logscale y 2
# set format y "2^{%L}"
# set ytics (0, 1,2,4,8,16,32,64,128,256,512, 1024, 2048, 4096, 8192, 16384, 32768, 65536)
# set yrange [0:1000]

set logscale x 2
set format x "2^{%L}"


# set ytics (400, 450, 500, 550, 600, 650, 700, 750, 800, 850, 900, 950, 1000)
set xtics (1, 3, 7, 15, 31, 63, 127, 255, 511, 1023, 2047, 4095, 8191, 16383, 32767, 65535, 131071, 262143)
set ytics 10

# Define line styles



# set style line 1 lc rgb '#dd181f' lt 1 lw 1.5 pt 7 ps 1.5
# set style line 2 lc rgb '#0060ad' lt 1 lw 1.5 pt 5 ps 1.5
set style line 3 lc rgb '#228B22' lt 1 lw 1.5 pt 9 ps 1.5

plot "test-on-raspberries.txt" using 1:4 with linespoints linestyle 3 title "Speedychain Disk with Stordy"