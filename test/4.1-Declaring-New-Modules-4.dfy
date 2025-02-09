module Mod {
  module Helpers {
    // the phrase 'function method' is not allowed 
    // when using --function-syntax:4; to declare a compiled function, 
    // use just 'function'
    function method addOne(n: nat): nat {
      n + 1
    }
  }
  method m() {
    var x := 5;
    x := Helpers.addOne(x); // x is now 6
  }
}