class CProgram {
  constructor(){
  }
  method main() returns (ret: int)
    requires true
    decreases *
    modifies this
  {
    var x : bv32 := 0;
    var y : bv32 := *;
    while (x < 99) 
      decreases *
    {
      if ((y % 2) == 0) {
        x := (x + 2);
      } else {
        x := (x + 1);
      }
    }
    assert(((x % 2) == (y % 2)));
    return 0;
  }

}

