package ops

import "testing"

func TestParseLooprStatus(t *testing.T) {
	log := "noise\n---LOOPR_STATUS---\nSTATUS: COMPLETE\nEXIT_SIGNAL: true\nWORK_TYPE: code\nFILES_MODIFIED: 3\nERRORS: 0\nSUMMARY: all tasks done\n---END_LOOPR_STATUS---\n"
	status, ok := ParseLooprStatus(log)
	if !ok {
		t.Fatalf("expected status block")
	}
	if !status.ExitSignal || status.Status != "COMPLETE" {
		t.Fatalf("status parsing failed: %#v", status)
	}
	if status.FilesModified != 3 || status.ErrorCount != 0 {
		t.Fatalf("numeric parsing failed: %#v", status)
	}
	if status.WorkType != "code" || status.Summary != "all tasks done" {
		t.Fatalf("field parsing failed: %#v", status)
	}
}

func TestParseLooprStatusMissing(t *testing.T) {
	if _, ok := ParseLooprStatus("no status here"); ok {
		t.Fatalf("expected no status block")
	}
}
