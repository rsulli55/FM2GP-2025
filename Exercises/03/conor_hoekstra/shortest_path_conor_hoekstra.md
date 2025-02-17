### Overview

Asked ChatGPT for graph and visualization, lol.

![image](https://github.com/user-attachments/assets/c2baf6ef-73a7-4960-9c1b-ba35330f6909)

Using NetworkX:
![network_graph](https://github.com/user-attachments/assets/cb46fd0a-b032-4e2e-82d8-c11824200eee)

Using Graphviz:
![graph](https://github.com/user-attachments/assets/6ee32f00-24ed-458f-912b-dcf2bdc0866a)

And this is the Kap code to solve:
```apl
Inf ← 100 ⍝ fake infinity
m   ← 5 5⍴0 7 5 Inf Inf 7 Inf 0 3 6 5 Inf 0 4 2 0 3 4 0 Inf Inf 6 2 Inf 0
io:print o3:format m

m ⌊∙+ m ⍝ so simple
```

