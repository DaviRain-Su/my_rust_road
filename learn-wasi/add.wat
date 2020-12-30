(module 
    (import "console" "log" (func $log (param i32 )))
    (func $add (param i32 i32)
        get_local 0
        get_local 1
        i32.add
        call $log
    )
    (export "add" (func $add))
)