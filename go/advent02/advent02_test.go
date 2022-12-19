package advent02

import "testing"

type Case struct {
	a string
	b string
	s int
}

func TestComputeScore(t *testing.T) {
	cases := [9]Case{
		{a: "A", b: "X", s: 4},
		{a: "A", b: "Y", s: 8},
		{a: "A", b: "Z", s: 3},
		{a: "B", b: "X", s: 1},
		{a: "B", b: "Y", s: 5},
		{a: "B", b: "Z", s: 9},
		{a: "C", b: "X", s: 7},
		{a: "C", b: "Y", s: 2},
		{a: "C", b: "Z", s: 6},
	}

	for _, c := range cases {
		a, b, s := c.a, c.b, c.s
		score := ComputeScorePart1(a, b)
		if score != s {
			t.Fatalf("%s and %s got %d, expected %d", a, b, score, s)
		}
	}
}

func TestComputeScorePart2(t *testing.T) {
	cases := [9]Case{
		{a: "A", b: "X", s: 3 + 0},
		{a: "A", b: "Y", s: 1 + 3},
		{a: "A", b: "Z", s: 2 + 6},
		{a: "B", b: "X", s: 1 + 0},
		{a: "B", b: "Y", s: 2 + 3},
		{a: "B", b: "Z", s: 3 + 6},
		{a: "C", b: "X", s: 2 + 0},
		{a: "C", b: "Y", s: 3 + 3},
		{a: "C", b: "Z", s: 1 + 6},
	}

	for _, c := range cases {
		a, b, s := c.a, c.b, c.s
		score := ComputeScorePart2(a, b)
		if score != s {
			t.Fatalf("%s and %s got %d, expected %d", a, b, score, s)
		}
	}
}
