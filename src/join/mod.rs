
moddef::moddef!(
    flat(pub) mod {
        actions for cfg(feature = "alloc"),
        try_actions for cfg(feature = "alloc")
    },
    flat mod {
        maybe_done for cfg(feature = "alloc")
    }
);