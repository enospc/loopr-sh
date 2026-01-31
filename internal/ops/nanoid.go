package ops

import (
	"fmt"
	"io"
	"strings"
)

const (
	repoIDLength    = 6
	repoIDAlphabet  = "useandom26T198340PX75pxJACKVERYMINDBUSHWOLFGQZbfghjklqvwyzrict"
	sessionIDLength = 6
)

func generateNanoID(reader io.Reader, length int) (string, error) {
	const mask = byte(63)
	if length <= 0 {
		return "", fmt.Errorf("nanoid length must be positive")
	}
	out := make([]byte, 0, length)
	buf := make([]byte, length)
	for len(out) < length {
		need := length - len(out)
		if need > len(buf) {
			need = len(buf)
		}
		if _, err := io.ReadFull(reader, buf[:need]); err != nil {
			return "", err
		}
		for _, b := range buf[:need] {
			idx := int(b & mask)
			if idx >= len(repoIDAlphabet) {
				continue
			}
			out = append(out, repoIDAlphabet[idx])
			if len(out) == length {
				break
			}
		}
	}
	return string(out), nil
}

func validRepoID(value string) bool {
	if len(value) != repoIDLength {
		return false
	}
	for i := 0; i < len(value); i++ {
		if strings.IndexByte(repoIDAlphabet, value[i]) == -1 {
			return false
		}
	}
	return true
}
