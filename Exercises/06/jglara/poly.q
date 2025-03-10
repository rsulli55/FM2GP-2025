
/ Polynomial evaluation using horners rule
poly:{[x;cs] cs[0] {y+x*z}[;;x]/ 1_ cs }
poly[10;1 2 3 4 5] / 12345
poly[3;2 -6 2 -1] / 5
