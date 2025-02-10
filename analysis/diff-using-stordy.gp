set datafile separator tab

set title "Crescimento do Tamanho da Speedychain" font ",18"
set xlabel "Número de Transações" font ",16"
set ylabel "Tamanho (MB)" font ",16"
set key left top
set key font ",16"
set grid

set output "size-growth.pdf"

set logscale x 2
set format x "2^{%L}"
set xtics (0, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216)

set logscale y 2
set format y "2^{%L}"
set ytics (0, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536)

set style line 1 lc rgb '#dd181f' lt 1 lw 1.5 pt 7 ps 1.5  
set style line 2 lc rgb '#0060ad' lt 1 lw 1.5 pt 5 ps 1.5  

plot "data.txt" using 1:3 with linespoints linestyle 1 title "Speedychain sem Stordy", \
     "data.txt" using 1:($2+$4) with linespoints linestyle 2 title "Speedychain com Stordy + Stordy"
