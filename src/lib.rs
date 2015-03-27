/*
 * Copyright (c) 2015 Brandon Sanderson
 * Part of the genfsm library.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 *
 */
use std::borrow::Borrow;

///Represents the next state an FSM will enter
pub enum NextState<S,I>{
  ///Change the state and the executor that handles input
  ChangeState(Executor<S,I>, S),
  ///Continue with the same executor
  Continue(S),
  ///FSM has reached completion.  Any further attempts
  ///to handle input will result in no changes
  End(S)
}

///An input handler that executes changes on a state
pub type Executor<S,I> = fn(S, I) -> NextState<S,I>;

///The core of the GenFSM library.  Holds the Executor
///and current state of a state machine, and handles
///the processing of NextState values from Executors.
pub struct StateMachine<S, I> {
  ///The next Executor to handle input.  Will be None
  ///if this state machine has finished.
  next_executor:Option<Executor<S,I>>,
  ///The current state.  Should only ever be None when the process
  ///method is running.
  current_state:Option<S>
}

impl<S,I> StateMachine<S,I>{
  ///Processes new input, using the current executor
  ///to update state.
  pub fn process(&mut self, input_val:I){
    let mut next_state_opt:Option<NextState<S,I>> = None;
    let old_state = self.current_state.take().unwrap();
    match self.next_executor {
      None => {}
      Some(ref executor) =>
        next_state_opt = Some((*executor)(old_state, input_val))
    };
    match next_state_opt {
      None => {}
      Some(next_state) =>
        match next_state {
          NextState::ChangeState(exec, state) => {
            self.next_executor = Some(exec);
            self.current_state = Some(state);
          }
          NextState::Continue(state) => {
            self.current_state = Some(state);
          }
          NextState::End(state) => {
            self.next_executor = None;
            self.current_state = Some(state);
          }
        }
    }
  }
  pub fn is_complete(&self) -> bool{
    self.next_executor.is_none()
  }
  ///Extracts the current state from the state machine.
  pub fn extract_state(self) -> S {
    self.current_state.unwrap()
  }

  ///Creates a state machine starting with the given state.
  ///The executor given will be used to process input.
  pub fn new(exec:Executor<S,I>, state:S) -> StateMachine<S,I>{
    StateMachine{next_executor:Some(exec), current_state:Some(state)}
  }
}

impl<S,I> Borrow<S> for StateMachine<S, I> {
  fn borrow(&self) -> &S {
    self.current_state.as_ref().unwrap()
  }
}

impl<S:Clone, I> StateMachine<S,I>{
  ///Returns a clone of the current state
  pub fn clone_state(&self) -> S {
    self.current_state.clone().unwrap()
  }
}

#[test]
fn simple_storage(){
  let storer:Executor<Option<isize>, isize> = simple_storage_statefn;
  let mut state_machine:StateMachine<Option<isize>, isize> = StateMachine::new(storer, None);
  assert!(state_machine.clone_state() == None);
  state_machine.process(2);
  assert!(state_machine.clone_state() == Some(2));
  state_machine.process(0);
  assert!(state_machine.is_complete());
}

#[cfg(test)]
fn simple_storage_statefn(state:Option<isize>, input:isize) -> NextState<Option<isize>, isize> {
  if input == 0 {
    NextState::End(state)
  }else if input < 0 {
    NextState::Continue(None)
  }else{
    NextState::Continue(Some(input))
  }
}
