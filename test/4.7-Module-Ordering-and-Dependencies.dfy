import A = B
method m() {
	A.whatever();
}
module B {
	method whatever() {
		assert 1 == 1;
	}
}