verbatimtex%&latex
\documentclass{article}
%\usepackage[subscriptcorrection]{mathtime}
\begin{document}
etex

outputtemplate := "%j-%c.mps";

input trie

beginfig(1);
	trienode.a(0, "001001");
	trienode.b(-1, "10");
	trienode.c(1, "010");
	trienode.d(-1, "10");
	trienode.e(-1, "");
	children.a(b, c, 20);
	children.c(d, e, 20);
	drawnodes(a, b, c, d, e);

	label.lft(btex $\alpha$ etex, c.w);
	drawdblarrow a.ne + 10right -- a.se + 10right -- a.se + 10right + 20right + 20down;
	label.bot(btex $n_\alpha$ etex rotated -90, a.ne + 15right + 20down);
	drawdblarrow a.se + 10right + 20right + 20down -- a.se + 10right + 20right + (0, -52);
	label.bot(btex $c_\alpha$ etex rotated -90, a.se + 35right + 30down);
	drawdblarrow a.ne + 30right -- a.se + 30right -- a.se + (50,-20) -- a.se + (52,-32);
	label.bot(btex $h_\alpha$ etex rotated -90, a.ne + 35right + 20down);
	drawdblarrow a.ne + 50right -- a.se + 50right -- a.se + (70,-20) -- a.se + (70,-52);
	label.bot(btex $e_\alpha$ etex rotated -90, a.ne + 55right + 20down);
	drawdblarrow a.se + (80,-20) -- a.se + (80,-52);
	label.bot(btex $[i_\alpha\,\mathpunct{\ldotp\ldotp}j_\alpha]=[7\,\mathpunct{\ldotp\ldotp}10]$ etex rotated -90, a.ne + 87right + 50down);

	drawarrow a.nw + 10left -- a.sw + 10left -- a.sw + 30left + 20down  -- a.sw + 30left + 30down -- a.sw + 50left + 30down dashed withdots scaled .3;
	label.top(btex exit(00100100) etex, a.sw + (-65,-30));
endfig;

bye
