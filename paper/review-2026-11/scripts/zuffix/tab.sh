#!/bin/bash
# Tabulate cpu_time (column 4) of Google Benchmark CSVs by pattern-length argument.
# Usage: tab.sh <blew dir> <date> <text> <lengths...> -- <structures...>
B=$1; d=$2; t=$3; shift 3
Ls=()
while [ "$1" != "--" ]; do Ls+=("$1"); shift; done
shift
printf "%-34s" "$d $t"
for L in "${Ls[@]}"; do printf "%11s" "$L"; done
echo
for s in "$@"; do
	f=$(ls "$B/$d/"*".$t.$s.$d.csv" 2>/dev/null)
	[ -z "$f" ] && { printf "%-34s (missing)\n" "$s"; continue; }
	printf "%-34s" "$s"
	for L in "${Ls[@]}"; do
		v=$(awk -F, -v n="\"BM_run/$L\"" '$1==n {printf "%.4g", $4}' "$f")
		printf "%11s" "${v:--}"
	done
	echo
done
echo
