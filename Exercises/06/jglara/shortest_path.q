

half:{x div 2}
odd:{1 = x mod 2}
even:{0 = x mod 2}

// Generic power accumulative algorithm
power_acc:{[r;a;n;op]
 if[n=0;:r];
 while[1;
  if[odd[n];
   r: op[r;a];
   if[n=1; :r];
   ];
  n: half[n];
  a: op[a;a];
  ];
 }

power:{[a;n;op]
 while[even[n];
  a: op[a;a];
  n: half[n];
  ]

 if[n=1; :a];
 power_acc[a;op[a;a];half[n-1];op]
 }

// shortest path

sp:{[g]
 sp_op:{x {min x+y}/:\: flip y};
 power[g;count[g]-1;sp_op]}

// Solving shortest path in a simple diagram
// @startuml
// digraph foo {
// rankdir=LR
//  0 -> 1 [label = "7"]
//  0 -> 2 [label = "3"]
//  1 -> 2 [label = "1"]
//  1 -> 3 [label = "2"]
//  1 -> 4 [label = "6"]
//  2 -> 3 [label = "2"]
//  3 -> 4 [label = "4"]
//  1 -> 0 [label = "7"]
//  2 -> 0 [label = "3"]
//  2 -> 1 [label = "1"]
//  3 -> 1 [label = "2"]
//  4 -> 1 [label = "6"]
//  3 -> 2 [label = "2"]
//  4 -> 3 [label = "4"]

// }
// @enduml
g:5 5 #0 7 3 1000000 1000000 7 0 1 2 6 3 1 0 2 1000000 1000000 2 2 0 4 1000000 6 1000000 4 0
sp[g]
