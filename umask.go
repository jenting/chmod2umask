package umask

import (
	"fmt"
	"os"
)

// osFileModeToUmaskSymbol converts os.FileMode to umask symbolic output
func osFileModeToUmaskSymbol(perm os.FileMode) string {
	permS := perm.String()
	mask := ""
	umask := ""
	for i := 1; i < len(permS); i++ {
		if permS[i] != '-' {
			mask = mask + string(permS[i])
		}
		switch i {
		case 3:
			umask = fmt.Sprintf("u=%s,", mask)
			mask = ""
		case 6:
			umask = umask + fmt.Sprintf("g=%s,", mask)
			mask = ""
		case 9:
			umask = umask + fmt.Sprintf("o=%s", mask)
		}
	}
	return umask
}
