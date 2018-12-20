package main

import "testing"

func TestParseInput(t *testing.T) {
	cases := []struct {
		in   string
		want Input
	}{
		{"#1 @ 555,891: 18x12", Input{1, 555, 891, 18, 12}},
		{"#2 @ 941,233: 16x14", Input{2, 941, 233, 16, 14}},
	}

	for _, c := range cases {
		got := ParseInput(c.in)
		if got != c.want {
			t.Errorf("Part1(%v) == %v, want %v", c.in, got, c.want)
		}
	}
}
