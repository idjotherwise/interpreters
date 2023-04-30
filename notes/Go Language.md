# Interface
An interface specifies certain methods that a struct must implement in order to satisfy the interface. For example:
```go
type X interface {
	Name() string
	Double() int
}

type N interface {
	X
	Triple() int
}

struct Ns {
	Statements []N
}

// we must implement `Triple` on `Ns` now, since it contains an array of `N`s, which is itself an `X`, so we also must implement `Double` and `Name` So:

func (n *Ns) Double() int {
	return 2
}
func (n *Ns) Triple() int {
	return 3
}
func (n *Ns) Name() string {
	"Ifan"
}
```