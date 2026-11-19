#!/bin/bash
# Generating .mp for examples (from sets of strings)
./trie.rb -n <large-example.txt >large-example.mp
./trie.rb -n <large-example-new.txt >large-example-new.mp
./trie.rb -j <small-example.txt >small-example.mp
./trie.rb -n <counter-seba.txt >counter-seba.mp
# Patch to color new nodes in large-example-new
TEMP=$(mktemp)
TEMP1=$(mktemp)
patch large-example-new.mp large-example-new.patch
# Renumbering of the new nodes so that they match the old ones (beta is the only new internal node)
sed 's/alpha_{5}/beta/g' <large-example-new.mp >$TEMP
for ((i=6; i < 16; i++)); do
	a=$(echo 's/alpha_{'$i'}/alpha_{'$((i-1))'}/g')
	sed $a <$TEMP >$TEMP1      # No in-place substitution to avoid difference between -i option on MacOS and Linux
	mv $TEMP1 $TEMP
done
mv $TEMP large-example-new.mp 
rm $TEMP $TEMP1
# Generating .pdf from .mp
for x in *mp; do
	if [ "$x" != "trie.mp" ] ; then
		mptopdf $x
	fi
done

rm *.log
