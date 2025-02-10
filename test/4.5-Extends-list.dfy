module M {
	const a := 10
	const b := 10
	const c := 10
	export A reveals a
	export B reveals b
	export C reveals c extends A, B
}