verbatimtex %&latex
\documentclass{article}
\begin{document}
etex

input trie

beginfig(1);
	trienode.n(0,"0111");
%  -> zooo
	trienode.nzoooz(3,"101");
% zooozozo -> zooozozo
	trienode.nzooozozoz(3,"011");
% zooozozozzoo -> zooozozozzoo
	trienode.nzooozozozzooz(-1,"011");
% zooozozozzoozzoo -> zooozozozzoozzoo
	trienode.nzooozozozzooo(0,"");
% zooozozozzooo -> zooozozozzooo
	trienode.nzooozozozzoooz(-1,"100000");
% zooozozozzooozoz -> zooozozozzooozozzzzz
	trienode.nzooozozozzoooo(0,"");
% zooozozozzoooo -> zooozozozzoooo
	trienode.nzooozozozzooooz(-1,"001110");
% zooozozozzoooozz -> zooozozozzoooozzzoooz
	trienode.nzooozozozzooooo(-1,"01");
% zooozozozzoooooz -> zooozozozzooooozo
	children.nzooozozozzoooo(nzooozozozzooooz, nzooozozozzooooo, (12, 20));
	children.nzooozozozzooo(nzooozozozzoooz, nzooozozozzoooo, (22, 20));
	children.nzooozozoz(nzooozozozzooz, nzooozozozzooo, (32, 20));
	trienode.nzooozozoo(1,"11");
% zooozozooo -> zooozozoooo
	trienode.nzooozozooooz(-1,"0101110");
% zooozozoooozzozo -> zooozozoooozzozoooz
	trienode.nzooozozooooo(0,"1");
% zooozozooooo -> zooozozoooooo
	trienode.nzooozozooooooz(2,"110");
% zooozozoooooozoo -> zooozozoooooozooz
	trienode.nzooozozoooooozoozz(-1,"01");
% zooozozoooooozoozzzo -> zooozozoooooozoozzzo
	trienode.nzooozozoooooozoozo(-1,"11");
% zooozozoooooozoozooo -> zooozozoooooozoozooo
	children.nzooozozooooooz(nzooozozoooooozoozz, nzooozozoooooozoozo, (12, 20));
	trienode.nzooozozooooooo(2,"000");
% zooozozooooooozz -> zooozozooooooozzz
	trienode.nzooozozooooooozzzz(-1,"1");
% zooozozooooooozzzz -> zooozozooooooozzzzo
	trienode.nzooozozooooooozzzo(-1,"");
% zooozozooooooozzzo -> zooozozooooooozzzo
	children.nzooozozooooooo(nzooozozooooooozzzz, nzooozozooooooozzzo, (12, 20));
	children.nzooozozooooo(nzooozozooooooz, nzooozozooooooo, (32, 20));
	children.nzooozozoo(nzooozozooooz, nzooozozooooo, (42, 20));
	children.nzoooz(nzooozozoz, nzooozozoo, (82, 20));
	trienode.nzoooo(3,"1101");
% zooooooz -> zoooooozo
	trienode.nzoooooozoz(2,"1001");
% zoooooozozoz -> zoooooozozozzo
	trienode.nzoooooozozozzoz(1,"1");
% zoooooozozozzozo -> zoooooozozozzozo
	trienode.nzoooooozozozzozoz(-1,"");
% zoooooozozozzozoz -> zoooooozozozzozoz
	trienode.nzoooooozozozzozoo(-1,"000");
% zoooooozozozzozoozzz -> zoooooozozozzozoozzz
	children.nzoooooozozozzoz(nzoooooozozozzozoz, nzoooooozozozzozoo, (12, 20));
	trienode.nzoooooozozozzoo(1,"100");
% zoooooozozozzooo -> zoooooozozozzooozz
	trienode.nzoooooozozozzooozzz(-1,"011");
% zoooooozozozzooozzzz -> zoooooozozozzooozzzzoo
	trienode.nzoooooozozozzooozzo(-1,"110");
% zoooooozozozzooozzoo -> zoooooozozozzooozzoooz
	children.nzoooooozozozzoo(nzoooooozozozzooozzz, nzoooooozozozzooozzo, (12, 20));
	children.nzoooooozoz(nzoooooozozozzoz, nzoooooozozozzoo, (32, 20));
	trienode.nzoooooozoo(2,"1001");
% zoooooozoooz -> zoooooozooozzo
	trienode.nzoooooozooozzoz(0,"");
% zoooooozooozzoz -> zoooooozooozzoz
	trienode.nzoooooozooozzozz(-1,"0");
% zoooooozooozzozz -> zoooooozooozzozzz
	trienode.nzoooooozooozzozo(-1,"");
% zoooooozooozzozo -> zoooooozooozzozo
	children.nzoooooozooozzoz(nzoooooozooozzozz, nzoooooozooozzozo, (12, 20));
	trienode.nzoooooozooozzoo(1,"000");
% zoooooozooozzooz -> zoooooozooozzoozzz
	trienode.nzoooooozooozzoozzzz(-1,"1");
% zoooooozooozzoozzzzo -> zoooooozooozzoozzzzo
	trienode.nzoooooozooozzoozzzo(-1,"1");
% zoooooozooozzoozzzoo -> zoooooozooozzoozzzoo
	children.nzoooooozooozzoo(nzoooooozooozzoozzzz, nzoooooozooozzoozzzo, (12, 20));
	children.nzoooooozoo(nzoooooozooozzoz, nzoooooozooozzoo, (32, 20));
	children.nzoooo(nzoooooozoz, nzoooooozoo, (72, 20));
	children.n(nzoooz, nzoooo, (162, 20));
	drawnodes(n, nzoooz, nzooozozoz, nzooozozozzooz, nzooozozozzooo, nzooozozozzoooz, nzooozozozzoooo, nzooozozozzooooz, nzooozozozzooooo, nzooozozoo, nzooozozooooz, nzooozozooooo, nzooozozooooooz, nzooozozoooooozoozz, nzooozozoooooozoozo, nzooozozooooooo, nzooozozooooooozzzz, nzooozozooooooozzzo, nzoooo, nzoooooozoz, nzoooooozozozzoz, nzoooooozozozzozoz, nzoooooozozozzozoo, nzoooooozozozzoo, nzoooooozozozzooozzz, nzoooooozozozzooozzo, nzoooooozoo, nzoooooozooozzoz, nzoooooozooozzozz, nzoooooozooozzozo, nzoooooozooozzoo, nzoooooozooozzoozzzz, nzoooooozooozzoozzzo);
	fillnode.nzooozozoo withgreyscale .5;
	drawnode_.nzooozozoo;
	fillnode.nzooozozooooo withgreyscale .8;
	drawnode_.nzooozozooooo;
	fillnode.nzooozozooooz withgreyscale .5;
	drawnode_.nzooozozooooz;
	label(btex $\alpha_{0}$ etex, n.w + (-10, -2));
	label(btex $\alpha_{1}$ etex, nzoooz.e + (+10, -2));
	label(btex $\alpha_{2}$ etex, nzooozozoz.e + (+10, -2));
	label(btex $\alpha_{3}$ etex, nzooozozozzooo.w + (-10, -2));
	label(btex $\alpha_{4}$ etex, nzooozozozzoooo.w + (-10, -2));
	label(btex $\beta$ etex, nzooozozoo.w + (-10, -2));
	label(btex $\alpha_{5}$ etex, nzooozozooooo.w + (-10, -2));
	label(btex $\alpha_{6}$ etex, nzooozozooooooz.e + (+10, -2));
	label(btex $\alpha_{7}$ etex, nzooozozooooooo.w + (-10, -2));
	label(btex $\alpha_{8}$ etex, nzoooo.w + (-10, -2));
	label(btex $\alpha_{9}$ etex, nzoooooozoz.e + (+10, -2));
	label(btex $\alpha_{10}$ etex, nzoooooozozozzoz.e + (+10, -2));
	label(btex $\alpha_{11}$ etex, nzoooooozozozzoo.w + (-10, -2));
	label(btex $\alpha_{12}$ etex, nzoooooozoo.w + (-10, -2));
	label(btex $\alpha_{13}$ etex, nzoooooozooozzoz.e + (+10, -2));
	label(btex $\alpha_{14}$ etex, nzoooooozooozzoo.w + (-10, -2));

endfig;

bye
