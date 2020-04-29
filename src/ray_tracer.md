# Ray Tracer Challenge

## Chapter 1

No overloading of functions!?
- is_approx_eq(f64,f64) vs is_approx_eq(Tuple,Tuple)
- maybe implement a comparison trait so is_approx_eq becomes generic?
  - use ApproxEq trait to create is_approx_eq method and function

Rather than Tuple being used for Vector and Point, use types and a variant (enum)?

Difficulty finding the sqrt function! Eventually found in `use std::f64;`. Still some confusion over whether to use as a function or as a member.