//! MicroState
//!
//! A minimal Finite State Machine.
//!
//! Example:
//!
//! ```
//! #[macro_use] extern crate microstate;
//! microstate! {
//!     Simple { A }
//!     states { A, B }
//!
//!     next {
//!         A => B
//!         B => A
//!     }
//! }
//!
//! # fn main() {
//! use Simple::State::*;
//!
//! let mut machine = Simple::new();
//! assert_eq!(A, machine.state());
//! assert_eq!(Some(B), machine.next());
//! assert_eq!(Some(A), machine.next());
//! # }
//! ```

/// Create a new state machine
///
/// It takes a name, the initial value, all possible states
/// and the transitions.
///
/// See the main documentation for a proper example.
#[macro_export]
macro_rules! microstate (
  (
      $machine:ident { $initial:ident }
      states { $($states:ident),* }

      $($meth:ident {
          $($from:ident => $to:ident)*
      })*
  ) => (
      #[allow(non_snake_case)]
      pub mod $machine {
          #[derive(Clone,PartialEq,Eq,Debug)]
          pub enum State {
              #[doc(hidden)]
              __InvalidState__, // Just be able to match _ further down
              $($states),*
          }
          #[derive(PartialEq,Eq,Debug)]
          pub struct Machine {
              state: State,
          }

          pub fn new() -> Machine {
              Machine::new()
          }

          impl Machine {
              pub fn new() -> Machine {
                  Machine {
                      state: State::$initial
                  }
              }

              pub fn state(&self) -> State {
                  self.state.clone()
              }

              $(pub fn $meth(&mut self) -> Option<State> {
                  match self.state {
                      $( State::$from => { self.state = State::$to; Some(State::$to) } ),*
                          _ => None
                  }
              })*
          }
      }
));

#[cfg(test)]
mod tests {
    microstate!{
        Micro { New }
        states { New, Confirmed, Ignored }

        confirm {
            New => Confirmed
        }

        ignore {
            New => Ignored
        }

        reset {
            Confirmed => New
            Ignored   => New
        }
    }
    use self::Micro::State::*;

    #[test]
    fn test_transition_works() {
        let mut machine = Micro::new();

        assert_eq!(New, machine.state());

        assert_eq!(Some(Confirmed), machine.confirm());
        assert_eq!(Confirmed, machine.state());

        assert_eq!(None, machine.ignore());
        assert_eq!(Confirmed, machine.state());

        assert_eq!(Some(New), machine.reset());
        assert_eq!(New, machine.state());

        assert_eq!(Some(Ignored), machine.ignore());
        assert_eq!(Ignored, machine.state());
    }
}
