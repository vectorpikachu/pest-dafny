method Outer(x: nat)
  decreases x
{
  // set y to an arbitrary non-negative integer
  var y :| 0 <= y;
  Inner(x, y);
}
method Inner(x: nat, y: nat)
  decreases x, y
{
  if y != 0 {
    Inner(x, y-1);
  } else if x != 0 {
    Outer(x-1);
  }
}