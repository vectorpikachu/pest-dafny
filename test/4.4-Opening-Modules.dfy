module Helpers {
  function addOne(n: nat): nat {
    n + 1
  }
}

module Mod {
  import opened Helpers
  function addOne(n: nat): nat {
    n - 1
  }
  method m() {
    assert addOne(5) == 6; // this is now false,
    // as this is the function just defined
    assert Helpers.addOne(5) == 6; // this is still true
  }
}