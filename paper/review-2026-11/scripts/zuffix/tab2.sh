#!/bin/bash
# Tabulate one column of Google Benchmark CSVs by pattern-length argument.
# Usage: tab2.sh <column name> <blew dir> <date> <text> <lengths...> -- <structures...>
COL=$1; B=$2; d=$3; t=$4; shift 4
Ls=()
while [ "$1" != "--" ]; do Ls+=("$1"); shift; done
shift
printf "%-32s" "$d $t [$COL]"
for L in "${Ls[@]}"; do printf "%10s" "$L"; done
echo
for s in "$@"; do
	f=$(ls "$B/$d/"*".$t.$s.$d.csv" 2>/dev/null)
	[ -z "$f" ] && { printf "%-32s (missing)\n" "$s"; continue; }
	printf "%-32s" "$s"
	for L in "${Ls[@]}"; do
		v=$(awk -F, -v L="$L" -v col="$COL" '
			NR==2 { for (i = 1; i <= NF; i++) { h = $i; gsub(/"/, "", h); if (h == col) c = i } }
			NR>2 { n = $1; gsub(/"/, "", n); split(n, a, "/"); if (a[1] == "BM_run" && a[2] == L && c) printf "%.4g", $c }' "$f")
		printf "%10s" "${v:--}"
	done
	echo
done
echo
