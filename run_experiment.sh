#!/bin/bash

MODE=3
ARR_SIZE=900000000

plot_result() {
  infile="$1"
  outfile="$2"
  echo " \
  set terminal 'pdf'; \
  set output '$outfile'; \
  set ylabel 'runtime (in sec)'; \
  set xlabel '#threads'; \
  plot '$infile' using 1:2 every ::0 with linespoints notitle;" | gnuplot
}

run_program_over_threads() {
  out_file="data_m${MODE}_n${ARR_SIZE}"
  out_plot="${out_file}_plot.pdf"
  tmp_file="tmp_out"
  echo -e "Threads\tRuntime\tSum" > $out_file

  for t in {1..8}
  do
    echo -e "\nRunning for $t thread(s):-"
    echo -ne "$t\t" >> $out_file

    # use as many cores as the number of threads for better performance
    cores=`seq -s "," 1 1 $t`
    ./main -m $MODE -n $ARR_SIZE -t $t -c $cores | tee $tmp_file
    awk '/Time elapsed/ {printf("%f\t",$3)} /Grand Sum/ {print $3}' $tmp_file >> $out_file
  done
  rm -f $tmp_file

  plot_result $out_file $out_plot
}

run_program_over_threads
