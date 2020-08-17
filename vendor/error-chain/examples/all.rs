#[macro_use]
extern crate error_chain;

pub mod inner {
    error_chain!{}
}

#[cfg(feature = "a_feature")]
pub mod feature {
    error_chain!{}
}

error_chain! {
    // Types generated by the macro. If empty or absent, it defaults to
    //     Error, ErrorKind, Result;
    types {
        // With custom names:
        MyError, MyErrorKind, MyResult;
        // Without the `Result` wrapper:
        //     Error, ErrorKind;
    }

    // Automatic bindings to other error types generated by `error_chain!`.
    links {
        Inner(inner::Error, inner::ErrorKind);
        // Attributes can be added at the end of the declaration.
        Feature(feature::Error, feature::ErrorKind) #[cfg(feature = "a_feature")];
    }

    // Bindings to types implementing std::error::Error.
    foreign_links {
        Io(::std::io::Error);
    }
}

fn main() {}
