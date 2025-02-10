set datafile separator tab
set title "Tempo para adicionar uma transação" font ",18"
set xlabel "Número de Transações" font ",16"
set ylabel "Tempo (ms)" font ",16"
set key left top
set key font ",16"
set grid
set logscale x 2
set format x "2^{%L}"
set xtics (1, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216) font ",10"
set yrange [0:5]
set ytics 1 font ",10"
set style line 1 lc rgb '#0060ad' lt 1 lw 2
plot "data.txt" using 1:6 smooth unique with lines linestyle 1 title "Tempo de Transação"