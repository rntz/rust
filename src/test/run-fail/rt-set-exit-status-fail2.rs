// error-pattern:whatever

struct r {
  x:int,
}

// Setting the exit status after the runtime has already
// failed has no effect and the process exits with the
// runtime's exit code
impl r : Drop {
    fn finalize() {
        os::set_exit_status(50);
    }
}

fn r(x:int) -> r {
    r {
        x: x
    }
}

fn main() {
    log(error, ~"whatever");
    do task::spawn {
      let i = r(5);
    };
    fail;
}
