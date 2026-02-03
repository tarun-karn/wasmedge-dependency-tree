(module
  (import "math" "add" (func $add (param i32 i32) (result i32)))

  (func (export "calculate") (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $add
  )
)
