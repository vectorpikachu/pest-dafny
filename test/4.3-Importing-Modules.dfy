module Helpers {
  function addOne(n: nat): nat {
    n + 1
  }
}
module Mod {
  import A = Helpers
  method m() {
    assert A.addOne(5) == 6;
  }
}