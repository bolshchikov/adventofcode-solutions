package main

import "testing"

func TestPart1(t *testing.T) {
	cases := []struct {
		in   []int
		want int
	}{
		{[]int{1, 2, -3, 1}, 1},
		{[]int{0}, 0},
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
		in   []int
		want int
	}{
		{[]int{+1, -1}, 0},
		{[]int{+3, +3, +4, -2, -4}, 10},
	}

	for _, c := range cases {
		got := Part2(c.in)
		if got != c.want {
			t.Errorf("Part2(%v) == %v, want %v", c.in, got, c.want)
		}
	}
}
