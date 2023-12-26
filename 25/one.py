
# this is basically
# a graph problem

# want to find the cut
# with the fewest number of edges crossing it


# in this case, know the number of wires is 3
# max-flow, min-cut

# idea: union-find style;
# - find the union that reduces the out degr

# idea: use max-flow min cut

in = set()
out = set()



while out_degree > 3:
    pull the element with smallest 
