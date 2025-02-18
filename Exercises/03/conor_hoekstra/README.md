### Overview

Asked ChatGPT for graph and visualization, lol.

![image](https://github.com/user-attachments/assets/bbb0d83c-4bda-4bac-80e3-d81eeca65391)

Using NetworkX:

![image](https://github.com/user-attachments/assets/cdc160e7-8da9-4372-b876-5efe169acb40)

Using Graphviz:

![image](https://github.com/user-attachments/assets/3d431014-34ed-4e3a-8ee7-03ae2499a392)

And this is the Kap code to solve:
```apl
Inf ← 100 ⍝ fake infinity
m   ← 5 5⍴0 7 5 Inf Inf 7 Inf 0 3 6 5 Inf 0 4 2 0 3 4 0 Inf Inf 6 2 Inf 0
io:print o3:format m

m ⌊∙+ m ⍝ so simple
```

