# Ray Tracer Challenge

## Chapter 1

No overloading of functions!?
- is_approx_eq(f64,f64) vs is_approx_eq(Tuple,Tuple)
- maybe implement a comparison trait so is_approx_eq becomes generic?
  - use ApproxEq trait to create is_approx_eq method and function

Rather than Tuple being used for Vector and Point, use types and a variant (enum)?

Difficulty finding the sqrt function! Eventually found in `use std::f64;`. Still some confusion over whether to use as a function or as a member.

normalized(tuple) uses abs(tuple) which has moved the tuple.
- easy to take references as parameters: `fn magnitude(tuple: &Tuple) -> f64`
- and the call site needs to make the reference explicit: `magnitude(&make_point(0.0,3.0,-4.0))`
- easy to forget to make parameters references until the compiling of uses fails

fiddly making trait function work with references
- because it forces all implementers to use references
- and all callsites to provide references
- not sure how the built in binary and unary functions allow the use of values
  - are those operators moving, and using up, the values?

