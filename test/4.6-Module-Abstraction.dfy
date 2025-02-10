module Interface {
  function addSome(n: nat): nat
    ensures addSome(n) > n
}
abstract module Mod {
  import A : Interface
  method m() {
    assert 6 <= A.addSome(5);
  }
}
module Implementation {
  function addSome(n: nat): nat
    ensures addSome(n) == n + 1
  {
    n + 1
  }
}

module Mod2 refines Mod {
  import A = Implementation
  method m() {
    assert 6 <= A.addSome(5);
  }
}