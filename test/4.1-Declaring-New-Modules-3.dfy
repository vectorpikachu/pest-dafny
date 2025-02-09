module Mod {
  module Helpers {
    class C {
      constructor() {
        this.f := 3;
      }
      method doIt()
      var f: int
    }
  }
  method m() {
    var x := new Helpers.C();
    x.doIt();
    x.f := 4;
  }
}