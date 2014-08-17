open Core.Std

(* Hashtable with character array keys *)
module Key = struct
  module T = struct
    type t = char Array.t with sexp
    let compare = compare
    let hash = Hashtbl.hash
  end
  include T
  include Hashable.Make (T)
end

(* Set of strings *)
module StringSet = Set.Make(String);;

(* Convert the input argument to a sorted char array *)
let get_letters str =
  let ary = String.to_array str in
  begin
    Array.sort ~cmp:compare ary;
    ary
  end

(* Read the anagram dictionary, creating a hashtable *)
let load_dictionary () =
  let add_line tbl line =
    begin
      match String.split ~on:' ' line with
      | key::value -> Hashtbl.set tbl ~key:(String.to_array key) ~data:(StringSet.of_list value)
      | _          -> invalid_arg "string must be at least two words"
    end;
    tbl
  in
  In_channel.with_file "anadict.txt"
    ~f:(fun t -> In_channel.fold_lines t ~init:(Key.Table.create ()) ~f:add_line)

(* Fold fcn over all r-length combinations of values *)
let each_combination ~values ~r ~init ~fcn =
  let length = Array.length values in
  if r = 0 || r > length
  then init
  else
    let indices     : int array = Array.init r ~f:(fun i -> i)
    and combination : 'a array  = Array.init r ~f:(fun i -> values.(i))
    in

    let rec loop acc =

      let rec bumpable i =
        if indices.(i) < length - r + i then Some i
        else if i = 0 then None
        else bumpable (i-1)

      and bump i =
        if i < r then begin
          indices.(i) <- indices.(i-1) + 1;
          bump (i+1)
        end

      and copy i =
        if i < r then begin
          combination.(i) <- values.(indices.(i));
          copy (i+1)
        end

      and next = fcn acc combination r in

      match bumpable (r-1) with
      | Some j -> begin
        indices.(j) <- indices.(j) + 1; bump (j+1); copy j;
        loop next
      end
      | None -> next

    in loop init

let main () =
  let t = load_dictionary ()
  and l = get_letters Sys.argv.(1)   (* "asdwtribnowplfglewhqagnbe" *)
  in
  let handle acc combo r =
    match Hashtbl.find t combo with
    | None -> acc
    | Some lst -> StringSet.union acc lst
  in
  let rec loop acc i =
    if i < Array.length l + 1
    then loop (each_combination ~values:l ~r:i ~init:acc ~fcn:handle) (i+1)
    else acc
  in
  let x = loop StringSet.empty 0
  in printf "%d\n" (StringSet.length x)

let _ = main ()
