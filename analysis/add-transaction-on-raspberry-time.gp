set datafile separator whitespace

set title "SpeedyChain Time To Add a Transaction in IoT Environment" font ",24"
set xlabel "Number of Transactions" font ",24"
set ylabel "Time to add a transaction (ms)" font ",24"

set key left top
set key font ",24"
set xrange [1024:300032]
set grid

set terminal pdfcairo size 11.7in,8.3in enhanced font "Arial,24"
set output "add-transaction-on-raspberry-time.pdf"

set logscale x 2
set format x "2^{%L}"

set ytics 20

set style line 1 lc rgb '#dd181f' lt 1 lw 3 pt 7 ps 1.5
set style line 2 lc rgb '#0060ad' lt 1 lw 3 pt 5 ps 1.5

plot  "add-transaction-on-raspberry-with-stordy.txt" using 1:2 smooth bezier linestyle 1 title "Time to add a transaction with Stordy", \
      "add-transaction-on-raspberry-without-stordy.txt" using 1:2 smooth bezier linestyle 2 title "Time to add a transaction without Stordy"
unset output
