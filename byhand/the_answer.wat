(module
    (import "imports" "notifyDone" (func $notify (param $result i32)))
    (import "imports" "iomem" (memory 1))

    (func (export "add2nums") (param $a i32) (param $b i32) (result i32)
        (call $notify (i32.add (local.get $a) (local.get $b)))
        (i32.load (i32.const 0))
    )
)
