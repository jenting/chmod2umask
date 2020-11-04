package umask

import (
	"os"
	"testing"
)

func TestOSFileModeToUmaskSymbol(t *testing.T) {
	tests := []struct {
		perm   os.FileMode
		expect string
	}{
		{
			perm:   os.FileMode(0777),
			expect: "u=rwx,g=rwx,o=rwx",
		},
		{
			perm:   os.FileMode(0642),
			expect: "u=rw,g=r,o=w",
		},
		{
			perm:   os.FileMode(0210),
			expect: "u=w,g=x,o=",
		},
	}

	for _, tt := range tests {
		t.Run(tt.perm.String(), func(t *testing.T) {
			got := osFileModeToUmaskSymbol(tt.perm)
			if got != tt.expect {
				t.Errorf("expected %s, got %s", tt.expect, got)
			}
		})
	}
}
