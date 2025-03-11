module type SemiRing = sig
  type t

  val zero : t
  val one : t
  val ( * ) : t -> t -> t
  val ( + ) : t -> t -> t
end

let odd i = Int.logand i 1 = 1
let half i = Int.shift_right i 1
let is_zero i = i = 0
let is_one i = i = 1
let pred = Int.pred

module Exponent (R : SemiRing) = struct
  include R

  let power (x : R.t) (n : int) =
    (* this is gross OCaml but it follows the algorithm *)
    let power_accumulate r a n =
      if is_zero n then r
      else
        let exception Return of R.t in
        try
          let r = ref r in
          let n = ref n in
          let a = ref a in
          while true do
            if odd !n then (
              r := !r * !a;
              if is_one !n then raise (Return !r));
            n := half !n;
            a := !a * !a
          done;
          failwith "unreachable"
        with Return r -> r
    in

    let a = ref x in
    let n = ref n in
    while not (odd !n) do
      a := !a * !a;
      n := half !n
    done;
    if is_one !n then !a else power_accumulate !a (!a * !a) (half (pred !n))
end

module Array = struct
  include Array

  let init_matrix m n f =
    (* in OCaml >= 5.2.0 Array.init_matrix is in standard library *)
    Array.init m (fun i -> Array.init n (fun j -> f i j))
end

module type Num = sig
  val n : int
end

module MatrixSemiRing (R : SemiRing) (N : Num) = struct
  open N

  type t = R.t array array

  let zero = Array.make_matrix n n R.zero
  let one = Array.init_matrix n n (fun i j -> if i = j then R.one else R.zero)
  let ( + ) x y = Array.(R.( + ) |> map2 |> map2) x y

  let ( * ) a b =
    let elt i j =
      let open R in
      Seq.init n (fun k -> a.(i).(k) * b.(k).(j)) |> Seq.fold_left ( + ) zero
    in
    Array.init_matrix n n elt
end

let inf = Float.infinity

module TropicalSemiRing = struct
  type t = float

  let zero = inf
  let one = 0.0
  let ( * ) = ( +. )
  let ( + ) = Float.min
end

let exercise_8_8 graph =
  let n = Array.length graph in
  let module M =
    MatrixSemiRing
      (TropicalSemiRing)
      (struct
        let n = n
      end)
  in
  let open Exponent (M) in
  power graph (n - 1)

module PathTropicalSemiRing = struct
  type t = float * string list

  let zero = (inf, [])
  let one = (0., [])
  let ( * ) (w, p) (w', p') = (w +. w', p @ p')
  let ( + ) (w, p) (w', p') = if w < w' then (w, p) else (w', p')
end

let label_nodes graph =
  (* if the graph has more than 26 nodes this will just fail, sorry *)
  let labels = "ABCDEFGHIJKLMNOPQRSTUVWXYZ" in
  let append_label i j e =
    ( e,
      if e = 0. || e = inf then []
      else [ Printf.sprintf "%c->%c" labels.[i] labels.[j] ] )
  in
  Array.mapi (fun i row -> Array.mapi (fun j e -> append_label i j e) row) graph

let exercise_8_9 graph =
  let n = Array.length graph in
  let module M =
    MatrixSemiRing
      (PathTropicalSemiRing)
      (struct
        let n = n
      end)
  in
  let open Exponent (M) in
  power graph (n - 1)

let print_matrix elt_to_string m =
  (* this isn't pretty but it does what it needs to *)
  let open Array in
  let print_row row =
    map elt_to_string row |> to_list |> String.concat " "
    |> Printf.printf "%s\n"
  in
  iter print_row m

let () =
  let graph =
    [|
      [| 0.; 6.; inf; 3.; inf; inf; inf |];
      [| inf; 0.; inf; inf; 2.; 10.; inf |];
      [| 7.; inf; 0.; inf; inf; inf; inf |];
      [| inf; inf; 5.; 0.; inf; 4.; inf |];
      [| inf; inf; inf; inf; 0.; inf; 3. |];
      [| inf; inf; 6.; inf; 7.; 0.; 8. |];
      [| inf; 9.; inf; inf; inf; inf; 0. |];
    |]
  in
  exercise_8_8 graph |> print_matrix Float.to_string;

  let graph = label_nodes graph in
  exercise_8_9 graph
  |> print_matrix (fun (w, p) ->
         Printf.sprintf "(%f [%s])" w @@ String.concat ", " p)
