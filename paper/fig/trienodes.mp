verbatimtex %&latex
\documentclass{article}
\begin{document}
etex

input boxes

beginfig(3);
	defaultdy:=5bp;
	defaultdx:=10bp;
	boxit.ia(btex $i_\alpha$ etex);
	boxit.ja(btex to the leaf etex);
	boxit.jm(btex $J^-$ etex);
	boxit.jp(btex $J^+$ etex);		
	boxit.lc(btex left etex);
	boxit.rc(btex right etex);
	ia.ne=ja.nw; ia.se=ja.sw;
	drawboxed(ia,ja);
	jm.ne=jp.nw; jm.nw=ia.sw; jp.ne=ja.se;
	%xpart(jm.e)=xpart(ia.e);
	ypart(lc.n)=ypart(jp.s);
	ypart(rc.n)=ypart(jp.s);
	ypart(lc.s)=ypart(rc.s);
	xpart(lc.e)=xpart(jm.e);
	xpart(rc.e)=xpart(jp.e);
	xpart(rc.w)=xpart(lc.e);
	xpart(lc.w)=xpart(ia.w);
	xpart(lc.e)-xpart(lc.w)=xpart(rc.e)-xpart(rc.w);
	drawboxed(jm,jp);
	drawboxed(lc,rc);
	
	drawarrow jm.c+(-10,0) {left} .. {down} jm.w+(-20,-50)  dashed withdots scaled .5;
	drawarrow jp.c+(+10,0) {right} .. {down} jp.e+(+20,-50)  dashed withdots scaled .5;
	drawarrow lc.s -- lc.s+(-5,-15);
	drawarrow rc.s -- rc.s+(+5,-15);
	drawarrow ja.e+(-10,0) -- ja.e+(10,0);
endfig;

beginfig(4);
	defaultdy:=15bp;
	boxjoin(a.se=b.sw; a.ne=b.nw);
	boxit.p(btex prev etex);
	boxit.pts(btex pointer to $S$ etex);
	boxit.ia(btex $i_\alpha$ etex);
	boxit.nd(btex to the internal node etex);
	boxit.n(btex next etex);
	drawboxed(p,nd,pts,ia,n);

	drawarrow p.w+(5,0) -- p.w+(-20,0);
	drawarrow n.e+(-5,0) -- n.e+(20,0);
	drawarrow pts.c+(0,-10) -- pts.c+(0,-30);
	drawarrow nd.c+(0,10) -- nd.c+(0,30);
endfig;

bye
