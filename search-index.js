var searchIndex = {};
searchIndex['genfsm'] = {"items":[[0,"","genfsm","",null,null],[3,"StateMachine","","The core of the GenFSM library.  Holds the Executor\nand current state of a state machine, and handles\nthe processing of NextState values from Executors.",null,null],[4,"NextState","","Represents the next state an FSM will enter",null,null],[13,"ChangeState","","Change the state and the executor that handles input",0,null],[13,"Continue","","Continue with the same executor",0,null],[13,"End","","FSM has reached completion.  Any further attempts\nto handle input will result in no changes",0,null],[6,"Executor","","An input handler that executes changes on a state",null,null],[11,"process","","Processes new input, using the current executor\nto update state.",1,{"inputs":[{"name":"statemachine"},{"name":"i"}],"output":null}],[11,"is_complete","","Returns true if there is no executor to handle additional input.",1,{"inputs":[{"name":"statemachine"}],"output":{"name":"bool"}}],[11,"extract_state","","Extracts the current state from the state machine.",1,{"inputs":[{"name":"statemachine"}],"output":{"name":"s"}}],[11,"new","","Creates a state machine starting with the given state.\nThe executor given will be used to process input.",1,{"inputs":[{"name":"statemachine"},{"name":"executor"},{"name":"s"}],"output":{"name":"statemachine"}}],[11,"borrow","","",1,{"inputs":[{"name":"statemachine"}],"output":{"name":"s"}}],[11,"clone_state","","Returns a clone of the current state",1,{"inputs":[{"name":"statemachine"}],"output":{"name":"s"}}]],"paths":[[4,"NextState"],[3,"StateMachine"]]};
initSearch(searchIndex);
