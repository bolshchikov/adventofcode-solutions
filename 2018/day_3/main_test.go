package main

import "testing"

func TestPart1(t *testing.T) {
	cases := []struct {
		in   []Input
		want int
	}{
		{[]Input{
			Input{1, 1, 3, 4, 4},
			Input{2, 3, 1, 4, 4},
			Input{3, 5, 5, 2, 2},
		}, 4},
	}

	for _, c := range cases {
		got := Part1(c.in)
		if got != c.want {
			t.Errorf("Part1(%v) == %v, want %v", c.in, got, c.want)
		}
	}
}

func TestPart2(t *testing.T) {
	cases := []struct {
		in   []Input
		want int
	}{
		{[]Input{
			Input{1, 1, 3, 4, 4},
			Input{2, 3, 1, 4, 4},
			Input{3, 5, 5, 2, 2},
		}, 3},
	}

	for _, c := range cases {
		got := Part2(c.in)
		if got != c.want {
			t.Errorf("Part1(%v) == %v, want %v", c.in, got, c.want)
		}
	}
}
