
/ iterative fibonacci
fib:{[n] last n {last[x], (+/)x }/0 1}

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


// fibnacci calculation using power
fib_op:{[n] power[2 2#1 1 1 0f;n-1;mmu][0;0]}


\ts fib_op 100000
\ts fib 100000
