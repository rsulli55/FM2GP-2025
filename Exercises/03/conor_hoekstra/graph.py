import graphviz

# Define the Graphviz graph
dot = graphviz.Graph(format="png")
dot.attr(dpi="300", rankdir="LR")


# Define edges with weights
edges = [
    ("KI", "RH", 7),
    ("KI", "AW", 5),
    ("RH", "JB", 3),
    ("RH", "HC", 6),
    ("AW", "JB", 4),
    ("AW", "HC", 2),
]

# Add nodes and edges
for u, v, w in edges:
    dot.edge(u, v, label=str(w))

# Set the graph size to double its default size
dot.attr(size="32,32")  # Default size is 8, so double it

# Render and save the graph as PNG
output_filename = "graph"
dot.render(output_filename)

print(f"Graph saved as {output_filename}.png")
