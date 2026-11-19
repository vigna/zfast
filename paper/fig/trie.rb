#!/usr/bin/ruby

require 'optparse'	

options = {}
options[:spacing] = 5

OptionParser.new do |opts|
	opts.banner = "Usage: " + ARGV[0] + " [options] [SPACING]"
	
	opts.on("-a", "--handles", "Show handles also on leaves") do |v|
		options[:handles] = v
	end

	opts.on("-n", "--nodes", "Put names on internal nodes") do |v|
		options[:nodes] = v
	end

	opts.on("-j", "--jump", "Show jump pointers") do |v|
		options[:jump] = v
	end

	opts.on("-l", "--leaves", "Put names on leaves") do |v|
		options[:leaves] = v
	end

	opts.on("-sX", "--spacing=X", "Set spacing to X (default: 5)") do |v|
		options[:spacing] = v.to_i
	end

	opts.on("-h", "--help", "Prints the help") do 
		puts opts
		exit
	end
end.parse!

 

SPACING = options[:spacing]

s = []
allnodes = []

$stdin.each_line do |l|
	s <<= l.strip
end

s.sort!

def lcp s,t
	i = 0
	[s.length, t.length].min.times do
		if s[i] != t[i]; then break; end
		i += 1
	end

	return s[0...i]
end

def msb x
	return (Math.log(x) / Math.log(2)).floor
end

def twofattest a,b
	if a < 0; then return 0; end
	return (-1 << msb(a ^ b)) & b
end

class Node
	attr_accessor :left, :right, :extent_length, :handle_length, :parent_extent_length, :is_leaf, :name, :left_attach, :right_attach;
	def initialize
		@left_attach = @right_attach = 0
	end

	def intercepts jump_length
		return jump_length > @parent_extent_length && ( @is_leaf || jump_length <= @extent_length )
	end

	def handle_length
		return twofattest( @parent_extent_length, @extent_length )
	end

	def jump_length
		if handle_length == 0; then return 1.0/0; end
		return handle_length + ( handle_length & - handle_length )
	end
end

def output_node start,t,prefix,allnodes,leaf_handles
	r = t[0]
	t.each do |x|
		r = lcp(r,x)
	end

	node = Node.new
	node.extent_length = prefix.length + r.length
	node.parent_extent_length = prefix.length - 1
	node.name = prefix.gsub(/0/, "z").gsub(/1/, "o")

	f = node.handle_length
	
	hl = (f - prefix.length)
	if (t.size == 1 && !leaf_handles)
		hl = -1
	end

	puts("\ttrienode.n" + prefix + "(" + hl.to_s + ",\"" + r + "\");");
	allnodes <<= "n" + prefix

	r.gsub!(/0/, "z");
	r.gsub!(/1/, "o")

	puts("% " + (node.name + r)[0...f] + " -> " + node.name + r )

	if t.size == 1; then 
		node.is_leaf = true
		return node, SPACING; 
	end

	lcplength = r.length

	left = []
	t.each do |x|
		if x[lcplength,1] == "0"; then left <<= x[lcplength+1..-1]; end
	end
	
	node.left, width_left = output_node(start + lcplength + 1, left, prefix + r + "z", allnodes, leaf_handles)

	right = []
	t.each do |x|
		if x[lcplength,1] == "1"; then right <<= x[lcplength+1..-1]; end
	end
	
	node.right, width_right = output_node(start + lcplength + 1, right, prefix + r + "o", allnodes, leaf_handles)

	puts("\tchildren.n" + prefix + "(n" + prefix + r + "z, n" + prefix + r + "o, (" + ( [ width_left + width_right ].max + SPACING/2 ).to_s + ", 20));")
	return node, width_left + width_right + SPACING
end

def draw_jumps node
	if node.is_leaf; then return; end

	t = node.left
	while( ! t.intercepts(node.jump_length) ) do t = t.left; end
	
	puts( "\tdrawarrow 1/2[n" + node.name + ".w, n" + node.name + ".sw]{left} ... " + t.left_attach.to_s + "[n" + t.name + ".w, n"+ t.name + ".nw]{right} dashed withdots scaled .5;" )
	t.left_attach = ( t.left_attach + 1.0 ) / 2;

	t = node.right
	while( ! t.intercepts(node.jump_length) ) do t = t.right; end
	
	puts( "\tdrawarrow 1/2[n" + node.name + ".e, n" + node.name + ".se]{right} ... " + t.right_attach.to_s + "[n" + t.name + ".e, n" + t.name + ".ne]{left} dashed withdots scaled .5;" )
	t.right_attach = ( t.right_attach + 1.0 ) / 2;

	draw_jumps(node.left)
	draw_jumps(node.right);
end

def draw_labels node, prefix, index, suffix, label_left, label_leaves, label_nodes
	#if ( !node.is_leaf and label_nodes)
	#	puts("\t% $", node.handle, "$ & $\\alpha_{", index, "}\\\\")
	#end
	if ( node.is_leaf && !label_leaves ); then return index - 1; end
	if ( node.is_leaf && label_leaves || !node.is_leaf && label_nodes)
		if label_left
			puts("\tlabel(btex " + prefix + index.to_s + suffix + " etex, n" + node.name + ".w + (-10, -2));")
		else
			puts("\tlabel(btex " + prefix + index.to_s + suffix + " etex, n" + node.name + ".e + (+10, -2));")
		end
	end
	if ( ! node.is_leaf); then index = draw_labels(node.left, prefix, index + 1, suffix, false, label_leaves, label_nodes); index = draw_labels(node.right, prefix, index + 1, suffix, true, label_leaves, label_nodes); end
	return index 
end

puts("verbatimtex \%&latex\n\\documentclass{article}\n\\begin{document}\netex\n\ninput trie\n\nbeginfig(1);\n")

root, dummy = output_node(0, s, "", allnodes, options[:handles])

puts("\tdrawnodes(" + allnodes.join(", ") + ");")

if (options[:jump]); then draw_jumps(root); end

draw_labels(root, "$\\alpha_{", 0, "}$", true, options[:leaves], options[:nodes])

puts("\nendfig;\n\nbye\n")
