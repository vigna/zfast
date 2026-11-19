input boxes

% Returns a picture containing the string s stacked. The global variable h@#
% will contain the correct vertical offset for the specified handle.

vardef stack@#(expr handle)(text s) =
	save pic;
	save xwidth;
	picture pic;
	pic := nullpicture;
	addto pic also thelabel.bot("0", (0, 0));
	xwidth = xpart (urcorner pic - ulcorner pic);
	pic := nullpicture;
	if length s = 0:
		addto pic doublepath (-xwidth/2,ypart urcorner pic) -- (xwidth/2,ypart urcorner pic) withpen pencircle scaled 0 withcolor white
	fi;
	if handle = -1: h@# := infinity else: h@# := 0 fi;
	for i = 1 upto (length s):
		addto pic also thelabel.bot(substring(i-1, i) of s, (0, ypart lrcorner pic));
		if i = handle: h@# := ypart lrcorner pic; fi
	endfor;
	pic
enddef;

% Defines a trie node containing the string s and handle at given position (similarly to boxit).
% A handle of -1 means no handle.

vardef trienode@#(expr handle)(text s) =
	boxit@#(stack@#(handle, s));
enddef;

% Fills a node with given properties.

vardef fillnode@# text withwhat =
	save foo;
	picture foo;
	foo := pic @#;
	fill @#ne -- @#nw -- @#sw -- @#se -- cycle withwhat;
enddef;

% Set this to 0 to avoid drawing handles.

drawhandles := 1;

% Draws a trie node.

vardef drawnode_@# =
	%save p; ???
	picture p;
	pair width;
	p := pic @#;
	draw p;
	if (h@# <> infinity) and (drawhandles <> 0):
		width := 3/2(urcorner p - ulcorner p);
		draw (ulcorner p) - width + (0,h@#) + 1/2@#dy * up -- (urcorner p) + width + (0,h@#) + 1/2@#dy * up dashed evenly
	fi;
	drawboxes(@#);
enddef;

% Positions the given nodes as children of the node named @#. edge can be
% either a numeric, specifying an x/y offset, or an explicit (x,y) offset pair.

vardef children@#(suffix left, right)(expr edge) =
	save loffset, roffset;
	pair loffset, roffset;
	if numeric edge:
		loffset = (-edge,-edge);
		roffset = (edge,-edge)
	else:
		loffset = (-xpart edge, -ypart edge);
		roffset = (xpart edge, -ypart edge)
	fi;
	right.n = @#s + roffset;
	left.n = @#s + loffset;
	pair leftchildn@#, rightchildn@#;
	leftchildn@# := left.n;
	rightchildn@# := right.n;
enddef;

% Draws the edges joining the node named @# to its children.

vardef drawedges_@# =
	drawarrow @#s ... leftchildn@# + (0,1pt);
	drawarrow @#s ... rightchildn@# + (0,1pt);
	label.top("0", 2/3[@#s,leftchildn@#] + (-1pt,1pt));
	label.top("1", 2/3[@#s,rightchildn@#] + (1pt,1pt));
enddef;

% Draws a list of nodes.

def drawnodes(text t) =
	forsuffixes s=t: 
		drawnode_.s; 
		if known leftchildn.s:
			drawedges_.s;
		fi;
	endfor
enddef;
