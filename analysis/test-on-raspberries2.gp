set datafile separator whitespace

set title "Consumo de Disco da Blockchain em Ambiente IoT" font ",24"
set xlabel "Número de Transações" font ",24"
set ylabel "Tamanho do Disco (MB)" font ",24"

set key left top
set key font ",24"
set grid

set terminal pdfcairo size 11.7in,8.3in enhanced font "Arial,24"
set output "add-transaction-on-raspberry-disk.pdf"

set logscale x 2
set format x "2^{%L}"

set xtics (1, 3, 7, 15, 31, 63, 127, 255, 511, 1023, 2047, 4095, 8191, 16383, 32767, 65535, 131071, 262143)
set ytics 10

set style line 3 lc rgb '#228B22' lt 1 lw 1.5 pt 9 ps 1.5

plot "test-on-raspberries.txt" using 1:4 with linespoints linestyle 3 title "Disco da Blockchain com Stordy"
