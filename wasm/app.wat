(module
  (import "calc" "calculate" (func $calculate (param i32 i32) (result i32)))

  (func (export "run") (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $calculate
  )
)
