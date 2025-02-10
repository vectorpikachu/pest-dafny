function Fib(n: nat) : nat
  decreases n
{
  if n < 2 then n else Fib(n-2) + Fib(n-1)
}

method A(x: nat)
  decreases x, 1
{
  B(x);
}
method B(x: nat)
  decreases x, 0
{
  if x != 0 { A(x-1); }
}