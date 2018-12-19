package main

import "testing"

func TestPart1(t *testing.T) {
	cases := []struct {
		in   []string
		want int
	}{
		{[]string{"abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"}, 12},
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
		in   []string
		want string
	}{
		{[]string{
			"abcde",
			"fghij",
			"klmno",
			"pqrst",
			"fguij",
			"axcye",
			"wvxyz",
		}, "fgij"},
	}

	for _, c := range cases {
		got := Part2(c.in)
		if got != c.want {
			t.Errorf("Part1(%v) == %v, want %v", c.in, got, c.want)
		}
	}
}
