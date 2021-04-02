extern crate autocfg;

macro_rules! feature_probe {
    ($probe_cfg:expr, $feature_cfg:expr, $ac:ident, $probe:expr, $($feature:expr),+,) => {
        let mut requires_feature = false;
        $($ac.set_feature($feature);)+
        if $probe {
            autocfg::emit($probe_cfg);
            requires_feature = true;
        }
        $($ac.unset_feature($feature);)+
        if $probe {
            if !requires_feature {
                autocfg::emit($probe_cfg);
            }
            requires_feature = false;
        }
        if requires_feature {
            autocfg::emit($feature_cfg);
        }
    }
}

fn main() {
    let mut ac = autocfg::new();

    feature_probe! {
        "impl_index",
        "feature_re_rebalance_coherence",
        ac,
        ac.probe_expression("{
            struct Foo;
            impl core::ops::Index<Foo> for str {
                type Output = str;
                fn index(&self, _: Foo) -> &str {
                    self
                }
            }
        }"),
        "re_rebalance_coherence",
    }

    feature_probe! {
        "impl_range_bounds",
        "feature_collections_range",
        ac,
        ac.probe_trait("core::ops::RangeBounds<usize>")
        && ac.probe_expression("core::ops::RangeBounds::start_bound(&(0..))")
        && ac.probe_expression("core::ops::RangeBounds::end_bound(&(0..))"),
        "collections_range",
    }

    feature_probe! {
        "impl_iterator",
        "feature_step",
        ac,
        ac.probe_trait("core::iter::Step")
        && ac.probe_expression("core::iter::Step::forward(1, 1)")
        && ac.probe_expression("core::iter::Step::forward_checked(1, 1)")
        && ac.probe_expression("unsafe { core::iter::Step::forward_unchecked(1, 1) }")
        && ac.probe_expression("core::iter::Step::backward_checked(1, 1)")
        && ac.probe_expression("unsafe { core::iter::Step::backward_unchecked(1, 1) }")
        && ac.probe_expression("core::iter::Step::steps_between(&1, &2)"),
        "step_trait",
        "step_trait_ext",
        "unchecked_math",
    }

    feature_probe! {
        "impl_trusted_len",
        "feature_trusted_len",
        ac,
        ac.probe_trait("core::iter::TrustedLen"),
        "trusted_len",
    }

    feature_probe! {
        "alloc",
        "feature_alloc",
        ac,
        ac.probe_sysroot_crate("alloc"),
        "alloc",
    }

    feature_probe! {
        "doc_cfg",
        "feature_doc_cfg",
        ac,
        ac.probe_expression(
            "{
                #[doc(cfg(foo))]
                struct Foo;
            }",
        ),
        "doc_cfg",
    }

    if ac.probe_sysroot_crate("std") {
        autocfg::emit("std");
    }
}
