package version

// These values can be overridden at build time via -ldflags.
var (
	Version = "dev"
	Commit  = ""
	Date    = ""
)
