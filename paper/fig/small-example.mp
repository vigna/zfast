verbatimtex %&latex
\documentclass{article}
\begin{document}
etex

input trie

beginfig(1);
	trienode.n(0,"0100");
%  -> zozz
	trienode.nzozzz(0,"");
% zozzz -> zozzz
	trienode.nzozzzz(0,"0");
% zozzzz -> zozzzzz
	trienode.nzozzzzzz(-1,"01111");
% zozzzzzz -> zozzzzzzzoooo
	trienode.nzozzzzzo(-1,"");
% zozzzzzo -> zozzzzzo
	children.nzozzzz(nzozzzzzz, nzozzzzzo, (12, 20));
	trienode.nzozzzo(0,"");
% zozzzo -> zozzzo
	trienode.nzozzzoz(-1,"");
% zozzzoz -> zozzzoz
	trienode.nzozzzoo(-1,"0100");
% zozzzooz -> zozzzoozozz
	children.nzozzzo(nzozzzoz, nzozzzoo, (12, 20));
	children.nzozzz(nzozzzz, nzozzzo, (32, 20));
	trienode.nzozzo(0,"");
% zozzo -> zozzo
	trienode.nzozzoz(-1,"00");
% zozzozzz -> zozzozzz
	trienode.nzozzoo(0,"");
% zozzoo -> zozzoo
	trienode.nzozzooz(-1,"00100");
% zozzoozz -> zozzoozzzozz
	trienode.nzozzooo(0,"");
% zozzooo -> zozzooo
	trienode.nzozzoooz(-1,"000");
% zozzoooz -> zozzooozzzz
	trienode.nzozzoooo(-1,"000");
% zozzoooo -> zozzoooozzz
	children.nzozzooo(nzozzoooz, nzozzoooo, (12, 20));
	children.nzozzoo(nzozzooz, nzozzooo, (22, 20));
	children.nzozzo(nzozzoz, nzozzoo, (32, 20));
	children.n(nzozzz, nzozzo, (72, 20));
	drawnodes(n, nzozzz, nzozzzz, nzozzzzzz, nzozzzzzo, nzozzzo, nzozzzoz, nzozzzoo, nzozzo, nzozzoz, nzozzoo, nzozzooz, nzozzooo, nzozzoooz, nzozzoooo);
	drawarrow 1/2[n.w, n.sw]{left} ... 0[nzozzzzzz.w, nzozzzzzz.nw]{right} dashed withdots scaled .5;
	drawarrow 1/2[n.e, n.se]{right} ... 0[nzozzoooo.e, nzozzoooo.ne]{left} dashed withdots scaled .5;
	drawarrow 1/2[nzozzz.w, nzozzz.sw]{left} ... 0[nzozzzz.w, nzozzzz.nw]{right} dashed withdots scaled .5;
	drawarrow 1/2[nzozzz.e, nzozzz.se]{right} ... 0[nzozzzo.e, nzozzzo.ne]{left} dashed withdots scaled .5;
	drawarrow 1/2[nzozzzz.w, nzozzzz.sw]{left} ... 0.5[nzozzzzzz.w, nzozzzzzz.nw]{right} dashed withdots scaled .5;
	drawarrow 1/2[nzozzzz.e, nzozzzz.se]{right} ... 0[nzozzzzzo.e, nzozzzzzo.ne]{left} dashed withdots scaled .5;
	drawarrow 1/2[nzozzzo.w, nzozzzo.sw]{left} ... 0[nzozzzoz.w, nzozzzoz.nw]{right} dashed withdots scaled .5;
	drawarrow 1/2[nzozzzo.e, nzozzzo.se]{right} ... 0[nzozzzoo.e, nzozzzoo.ne]{left} dashed withdots scaled .5;
	drawarrow 1/2[nzozzo.w, nzozzo.sw]{left} ... 0[nzozzoz.w, nzozzoz.nw]{right} dashed withdots scaled .5;
	drawarrow 1/2[nzozzo.e, nzozzo.se]{right} ... 0[nzozzoo.e, nzozzoo.ne]{left} dashed withdots scaled .5;
	drawarrow 1/2[nzozzoo.w, nzozzoo.sw]{left} ... 0[nzozzooz.w, nzozzooz.nw]{right} dashed withdots scaled .5;
	drawarrow 1/2[nzozzoo.e, nzozzoo.se]{right} ... 0.5[nzozzoooo.e, nzozzoooo.ne]{left} dashed withdots scaled .5;
	drawarrow 1/2[nzozzooo.w, nzozzooo.sw]{left} ... 0[nzozzoooz.w, nzozzoooz.nw]{right} dashed withdots scaled .5;
	drawarrow 1/2[nzozzooo.e, nzozzooo.se]{right} ... 0.75[nzozzoooo.e, nzozzoooo.ne]{left} dashed withdots scaled .5;

endfig;

bye
